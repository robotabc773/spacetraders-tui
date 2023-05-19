use std::sync::Arc;

use anyhow::Result;
use spacedust::apis::agents_api::get_my_agent;
use tokio::sync::Mutex;

use crate::{app::App, config::CONFIGURATION, st_util};

use super::IoEvent;

pub struct IoHandler {
    app: Arc<Mutex<App>>,
}

impl IoHandler {
    pub fn new(app: Arc<Mutex<App>>) -> Self {
        Self { app }
    }

    /// Handles a given [`IoEvent`] with the appropriate function
    ///
    /// # Errors
    /// Passes along any errors from the chosen function
    pub async fn handle_io_event(&mut self, io_event: IoEvent) -> Result<()> {
        match io_event {
            IoEvent::UpdateAgent => self.update_agent().await?,
            IoEvent::UpdateContracts => self.update_contracts().await?,
            IoEvent::UpdateFactions => self.update_factions().await?,
        }

        Ok(())
    }

    /// Updates information on the current agent from SpaceTraders API
    ///
    /// # Errors
    /// Errors on request failure
    pub async fn update_agent(&mut self) -> Result<()> {
        let agent = get_my_agent(&CONFIGURATION).await?.data;

        let mut app = self.app.lock().await;
        app.state.agent = *agent;

        // sqlx::query!(
        //     "INSERT INTO agents(account_id, symbol, headquarters, credits)
        //          VALUES ($1, $2, $3, $4)
        //          ON CONFLICT (symbol) DO
        //             UPDATE SET credits = EXCLUDED.credits",
        //     agent.account_id,
        //     agent.symbol,
        //     agent.symbol,
        //     agent.credits
        // )
        // .execute(get_global_db_pool().await)
        // .await?;

        Ok(())
    }

    /// Updates information on the current contracts from the SpaceTraders API
    ///
    /// # Errors
    /// Errors on request failure
    pub async fn update_contracts(&mut self) -> Result<()> {
        let contracts = st_util::list_contracts().await?;

        let mut app = self.app.lock().await;
        app.state.contracts = contracts;

        Ok(())
    }

    /// Updates information on the current factions from the SpaceTraders API
    ///
    /// # Errors
    /// Errors on request failure
    pub async fn update_factions(&mut self) -> Result<()> {
        let factions = st_util::list_factions().await?;

        let mut app = self.app.lock().await;
        app.state.factions = factions;

        Ok(())
    }
}
