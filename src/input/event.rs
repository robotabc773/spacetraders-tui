use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use log::error;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};
use tokio::sync::mpsc;

/// Terminal events.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug)]
pub enum InputEvent {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),
}

/// Terminal event handler.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    _sender: mpsc::Sender<InputEvent>,
    /// Event receiver channel.
    receiver: mpsc::Receiver<InputEvent>,
    /// Event handler thread.
    _handler: tokio::task::JoinHandle<()>,
    stop_capture: Arc<AtomicBool>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    #[must_use]
    #[allow(clippy::expect_used)]
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel(100);
        let stop_capture = Arc::new(AtomicBool::new(false));
        let handler = {
            let sender = sender.clone();
            let stop_capture = stop_capture.clone();
            tokio::spawn(async move {
                let mut last_tick = Instant::now();
                loop {
                    if stop_capture.load(Ordering::Relaxed) {
                        break;
                    }

                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("no events available") {
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Key(e) => sender.send(InputEvent::Key(e)).await,
                            CrosstermEvent::Mouse(e) => sender.send(InputEvent::Mouse(e)).await,
                            CrosstermEvent::Resize(w, h) => {
                                sender.send(InputEvent::Resize(w, h)).await
                            }
                            _ => unimplemented!(),
                        }
                        .expect("failed to send terminal event");
                    }

                    if last_tick.elapsed() >= tick_rate {
                        if !sender.is_closed() {
                            if let Err(err) = sender.send(InputEvent::Tick).await {
                                error!("Oops!, {err:#?}");
                            }
                        }
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self {
            _sender: sender,
            receiver,
            _handler: handler,
            stop_capture,
        }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    ///
    /// # Errors
    /// Returns `Err` if receiver is closed
    pub async fn next(&mut self) -> InputEvent {
        self.receiver.recv().await.unwrap_or(InputEvent::Tick)
    }

    pub fn close(&mut self) {
        self.stop_capture.store(true, Ordering::Relaxed);
    }
}
