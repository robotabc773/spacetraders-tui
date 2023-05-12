use anyhow::Result;
use spacetraders_tui::app::App;
use spacetraders_tui::event::{Event, EventHandler};
use spacetraders_tui::handler::handle_key_events;
use spacetraders_tui::tui::Tui;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> Result<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            // Event::Mouse(_) => {}
            // Event::Resize(_, _) => {}
            _ => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
