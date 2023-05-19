use log::error;
use spacedust::models::{Agent, Contract, Faction};
use strum::{Display, EnumCount, EnumIter};
use tokio::sync::mpsc;
use tui::widgets::ListState;

use crate::io::IoEvent;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    running: bool,
    /// Current application state
    pub state: State,
    io_sender: mpsc::Sender<IoEvent>,
}

#[derive(Debug)]
pub struct State {
    /// current main tab
    pub tab: Tab,
    /// current [`Agent`] data
    pub agent: Agent,
    /// current [`Contract`] data
    pub contracts: Vec<Contract>,
    /// [`ListState`] for list of Contracts on the agent page
    pub contracts_list_state: ListState,
    /// current [`Faction`] data
    pub factions: Vec<Faction>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            tab: Tab::Agent,
            agent: Agent::default(),
            contracts: Vec::new(),
            contracts_list_state: ListState::default(),
            factions: Vec::new(),
        }
    }
}

#[derive(Debug, EnumIter, Display, EnumCount, PartialEq, Eq)]
pub enum Tab {
    Agent,
    Systems,
    Fleet,
}

impl App {
    /// Constructs a new instance of [`App`].
    #[must_use]
    pub fn new(io_sender: mpsc::Sender<IoEvent>) -> Self {
        Self {
            running: true,
            state: State::default(),
            io_sender,
        }
    }

    pub async fn dispatch(&mut self, action: IoEvent) {
        if let Err(e) = self.io_sender.send(action).await {
            error!("Error from dispatch {e}");
        }
    }

    pub async fn update_agent_tab(&mut self) {
        self.dispatch(IoEvent::UpdateAgent).await;
        self.dispatch(IoEvent::UpdateContracts).await;
        self.dispatch(IoEvent::UpdateFactions).await;
    }

    pub async fn accept_or_fulfull_contract(&mut self) {
        if let Some(index) = self.state.contracts_list_state.selected() {
            let contract = &self.state.contracts[index];
            if !contract.accepted {
                self.dispatch(IoEvent::AcceptContract(contract.id.clone()))
                    .await;
            } else if let Some(delivers) = &contract.terms.deliver {
                if delivers
                    .iter()
                    .all(|d| d.units_fulfilled >= d.units_required)
                {
                    self.dispatch(IoEvent::AcceptContract(contract.id.clone()))
                        .await;
                }
            }
        }
    }

    fn list_move(&mut self, delta: i32) {
        let (count, list_state) = match self.state.tab {
            Tab::Agent => (
                self.state.contracts.len(),
                &mut self.state.contracts_list_state,
            ),
            _ => return,
        };
        if count == 0 {
            list_state.select(None);
        } else if let Some(selected) = list_state.selected() {
            list_state.select(Some(
                (i32::try_from(selected).unwrap_or(i32::MAX) + delta)
                    .rem_euclid(i32::try_from(count).unwrap_or(i32::MAX)) as usize,
            ));
        } else {
            list_state.select(Some(0));
        }
    }

    pub fn list_next(&mut self) {
        self.list_move(1);
    }

    pub fn list_prev(&mut self) {
        self.list_move(-1);
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Returns if the app is currently running
    #[must_use]
    pub fn running(&self) -> bool {
        self.running
    }

    // /// Returns the current app state
    // #[must_use]
    // pub fn state(&self) -> &State {
    //     &self.state
    // }
    //
    // /// Sets the current tab that the user is viewing
    // pub fn set_tab(&mut self, tab: Tab) {
    //     self.state.tab = tab;
    // }
    //
    // /// Sets the current agent info
    // pub fn set_agent(&mut self, agent: Agent) {
    //     self.state.agent = agent;
    // }
    //
    // /// Sets the current contract info
    // pub fn set_contracts(&mut self, contracts: Vec<Contract>) {
    //     self.state.contracts = contracts;
    // }
    //
    // /// Sets the current faction info
    // pub fn set_factions(&mut self, factions: Vec<Faction>) {
    //     self.state.factions = factions;
    // }
}
