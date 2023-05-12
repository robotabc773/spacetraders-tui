// Based on https://gist.github.com/Jules-Bertholet/7bf734b3593e8f9831ef279246358b12

use std::time::Duration;

use chrono::{DateTime, Utc};
use log::{error, warn};
use reqwest::{Request, Response, StatusCode};
use task_local_extensions::Extensions;
use tokio::{
    sync::{Mutex, Semaphore, SemaphorePermit},
    time::sleep,
};

/// The current documented burst limit.
static BURST_LIMIT: u16 = 10;

/// The current documented per-second limit.
static RATE_LIMIT: u16 = 2;

// Exponential backoff constants.

/// For exponential backoff retry on server errors.
const BACKOFF_CONSTANT_SECONDS: f64 = 10.0;
/// For exponential backoff retry on server errors.
const BACKOFF_BASE: f64 = 1.5;
/// For exponential backoff retry on server errors.
const BACKOFF_EXPONENT_INCREMENT: f64 = 1.0;

/// A permit must be acquired from this pool before any API request.
static REQUEST_SEMAPHORE: Semaphore = Semaphore::const_new(BURST_LIMIT as usize);

struct ReturnPermit;
impl ReturnPermit {
    /// Wait for the appropriate amount of time (calculated from the rate limit), then returns a permit to the pool.
    async fn return_permit(&mut self, permit: SemaphorePermit<'_>) {
        sleep(Duration::from_millis(
            1000_u16.saturating_div(RATE_LIMIT).into(),
        ))
        .await;

        drop(permit);
    }
}

/// This mutex must be locked before a permit is returned to the pool.
/// Locking ensures that waiting periods are sequential.
static RETURN: Mutex<ReturnPermit> = Mutex::const_new(ReturnPermit);

/// Middleware to enforce rate-limiting for the SpaceTraders API.
#[derive(Default)]
pub struct Middleware;

#[async_trait::async_trait]
impl reqwest_middleware::Middleware for Middleware {
    async fn handle(
        &self,
        mut reqest: Request,
        extensions: &mut Extensions,
        mut next: reqwest_middleware::Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        // Acquire a permit, yield if burst limit attained.
        #[allow(clippy::expect_used)]
        let permit = REQUEST_SEMAPHORE
            .acquire()
            .await
            .expect("sempahore is not closed");

        let mut retry_info;
        let mut server_error_retry_count: usize = 0;
        let result = loop {
            retry_info = reqest
                .try_clone()
                .map(|cloned_req| (cloned_req, next.clone()));

            match (retry_info, next.run(reqest, extensions).await) {
                // If this request isn't retryable, return the response no matter what it is.
                (None, resp) => break resp,

                // If this request was successful, return the response.
                (_, Ok(resp)) if resp.status().is_success() => break Ok(resp),

                // On server error, log and retry with exponential backoff.
                (Some((cloned_req, cloned_next)), Ok(resp)) if resp.status().is_server_error() => {
                    let status = resp.status();
                    let resp_text = resp.text().await.ok();
                    error!(
                        "Server error: {}. This request was previously tried {} times. Response body: {}",
                        status,
                        server_error_retry_count,
                        resp_text.as_deref().unwrap_or("<bytes>")
                    );
                    #[allow(clippy::cast_precision_loss)]
                    let backoff_exponent =
                        BACKOFF_EXPONENT_INCREMENT * server_error_retry_count as f64;
                    sleep(Duration::from_secs_f64(
                        BACKOFF_CONSTANT_SECONDS * BACKOFF_BASE.powf(backoff_exponent),
                    ))
                    .await;
                    server_error_retry_count += 1;
                    reqest = cloned_req;
                    next = cloned_next;
                }

                // If, despite our efforts, we've hit a rate limit, wait for the limits to reset and then retry.
                (Some((cloned_req, cloned_next)), Ok(resp))
                    if resp.status() == StatusCode::TOO_MANY_REQUESTS =>
                {
                    if let Some(reset) = resp
                        .headers()
                        .get("x-ratelimit-reset")
                        .and_then(|h| h.to_str().ok())
                        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                    {
                        warn!("Rate limit hit! Waiting and retrying.");

                        let delay = reset
                            .signed_duration_since(Utc::now())
                            .to_std()
                            .unwrap_or(Duration::ZERO);

                        sleep(delay).await;

                        reqest = cloned_req;
                        next = cloned_next;
                    } else {
                        break Ok(resp);
                    }
                }

                // Otherwise, pass on the error.
                (_, resp) => break resp,
            }
        };

        // Return permit to the pool after an appropriate timeout.
        tokio::spawn(async move {
            let mut return_permit = RETURN.lock().await;
            return_permit.return_permit(permit).await;
            drop(return_permit);
        });

        result
    }
}
