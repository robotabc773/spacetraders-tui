use crate::{
    app::{App, Tab},
    st_util::refresh_agent_page,
};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
///
/// # Errors
/// Currently never errors
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> Result<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c' | 'C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Tab-switching
        KeyCode::Char('a' | 'A') => {
            app.set_tab(Tab::Agent);
        }
        KeyCode::Char('s' | 'S') => {
            app.set_tab(Tab::Systems);
        }
        KeyCode::Char('f' | 'F') => {
            app.set_tab(Tab::Fleet);
        }
        // Tab-specific behavior
        key => match app.tab {
            Tab::Agent => match key {
                KeyCode::Char('r' | 'R') => {
                    tokio::spawn(refresh_agent_page());
                }
                _ => {}
            },
            Tab::Systems => {}
            Tab::Fleet => {}
        },
    }
    Ok(())
}
