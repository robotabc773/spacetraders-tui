use strum::{Display, EnumCount, EnumIter};

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// current main tab
    pub tab: Tab,
}

#[derive(Debug, EnumIter, Display, EnumCount, PartialEq, Eq)]
pub enum Tab {
    Agent,
    Systems,
    Fleet,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            tab: Tab::Agent,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn set_tab(&mut self, tab: Tab) {
        self.tab = tab;
    }
}
