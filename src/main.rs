use anyhow::Result;
use log::error;
use spacetraders_tui::app::App;
use spacetraders_tui::db_util;
use spacetraders_tui::input::event::{EventHandler, InputEvent};
use spacetraders_tui::input::handler::handle_key_events;
use spacetraders_tui::io::handler::IoHandler;
use spacetraders_tui::io::IoEvent;
use spacetraders_tui::tui::Tui;
use std::io;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tui::backend::CrosstermBackend;
use tui::Terminal;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Setup database stuff
    dotenvy::dotenv()?;
    db_util::setup_database().await;

    // Create IoEvent channel
    let (sync_io_sender, mut sync_io_reciever) = mpsc::channel::<IoEvent>(100);

    // Create an application.
    let app_ref = Arc::new(Mutex::new(App::new(sync_io_sender.clone())));

    // Spawn thread to handle I/O
    let io_app_ref = app_ref.clone();
    tokio::spawn(async move {
        let mut handler = IoHandler::new(io_app_ref);
        while let Some(io_event) = sync_io_reciever.recv().await {
            if let Err(e) = handler.handle_io_event(io_event).await {
                error!("Error handling io event: {e:#?}");
            }
        }
    });

    // Initialize internal state
    let mut app = app_ref.lock().await;
    app.update_agent_tab().await;
    drop(app);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    loop {
        let mut app = app_ref.lock().await;
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await {
            InputEvent::Tick => app.tick(),
            InputEvent::Key(key_event) => handle_key_events(key_event, &mut app).await?,
            // Event::Mouse(_) => {}
            // Event::Resize(_, _) => {}
            _ => {}
        }
        if !app.running() {
            tui.events.close();
            break;
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
