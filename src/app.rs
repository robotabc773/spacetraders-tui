use log::error;
use spacedust::models::Agent;
use strum::{Display, EnumCount, EnumIter};
use tokio::sync::mpsc;

use crate::io::IoEvent;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    running: bool,
    /// Current application state
    state: State,
    io_sender: mpsc::Sender<IoEvent>,
}

#[derive(Debug)]
pub struct State {
    /// current main tab
    pub tab: Tab,
    /// current [`Agent`] data
    pub agent: Agent,
}

impl Default for State {
    fn default() -> Self {
        Self {
            tab: Tab::Agent,
            agent: Agent::default(),
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

    /// Returns the current app state
    #[must_use]
    pub fn state(&self) -> &State {
        &self.state
    }

    /// Sets the current tab that the user is viewing
    pub fn set_tab(&mut self, tab: Tab) {
        self.state.tab = tab;
    }

    /// Sets the current agent info
    pub fn set_agent(&mut self, agent: Agent) {
        self.state.agent = agent;
    }
}
