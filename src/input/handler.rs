use crate::app::{App, Tab};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
///
/// # Errors
/// Currently never errors
pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> Result<()> {
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
        KeyCode::Char('1') => app.state.tab = Tab::Agent,
        KeyCode::Char('2') => app.state.tab = Tab::Systems,
        KeyCode::Char('3') => app.state.tab = Tab::Fleet,
        // List navigation
        KeyCode::Up => app.list_prev(),
        KeyCode::Down => app.list_next(),
        // Tab-specific behavior
        key => match app.state.tab {
            Tab::Agent => match key {
                KeyCode::Char('r' | 'R') => {
                    app.update_agent_tab().await;
                }
                KeyCode::Enter => {
                    app.accept_or_fulfull_contract().await;
                }
                _ => {}
            },
            Tab::Systems => {}
            Tab::Fleet => {}
        },
    }
    Ok(())
}
