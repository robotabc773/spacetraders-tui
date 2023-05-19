use std::sync::Arc;

use anyhow::Result;
use spacedust::apis::agents_api::get_my_agent;
use tokio::sync::Mutex;

use crate::{app::App, config::CONFIGURATION, st_util};

use super::IoEvent;

#[allow(clippy::module_name_repetitions)]
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
    /// Errors if the corresponding request fails
    pub async fn handle_io_event(&mut self, io_event: IoEvent) -> Result<()> {
        match io_event {
            IoEvent::UpdateAgent => self.update_agent().await?,
            IoEvent::UpdateContracts => self.update_contracts().await?,
            IoEvent::UpdateFactions => self.update_factions().await?,
            IoEvent::AcceptContract(id) => self.accept_contract(&id).await?,
            IoEvent::FulfillContract(id) => self.fulfill_contract(&id).await?,
        }

        Ok(())
    }

    async fn update_agent(&mut self) -> Result<()> {
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

    async fn update_contracts(&mut self) -> Result<()> {
        let contracts = st_util::list_contracts().await?;

        let mut app = self.app.lock().await;
        if contracts.is_empty() {
            app.state.contracts_list_state.select(None);
        } else {
            app.state.contracts_list_state.select(Some(0));
        }
        app.state.contracts = contracts;

        Ok(())
    }

    async fn update_factions(&mut self) -> Result<()> {
        let factions = st_util::list_factions().await?;

        let mut app = self.app.lock().await;
        app.state.factions = factions;

        Ok(())
    }

    async fn accept_contract(&mut self, id: &str) -> Result<()> {
        spacedust::apis::contracts_api::accept_contract(&CONFIGURATION, id, 0).await?;

        Ok(())
    }

    async fn fulfill_contract(&mut self, id: &str) -> Result<()> {
        spacedust::apis::contracts_api::fulfill_contract(&CONFIGURATION, id, 0).await?;

        Ok(())
    }
}
