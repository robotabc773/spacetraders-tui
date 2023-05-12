## File Structure

- main.rs - main function and event loop
- lib.rs - mod declarations
- app.rs - app state + custom Result (CHANGE ME FOR **APP STATE + TICK FUNCTION**)
- event.rs - tracks and stores terminal events
- handler.rs - handles terminal events (CHANGE ME FOR **INPUT HANDLING**)
- tui.rs - initializes/exists the terminal interface
- ui.rs - renders the UI (CHANGE ME FOR **RENDERING**)
- rate_limit.rs - middleware for rate limiting requests to SpaceTraders
- config.rs - global API configuration and database pool
- st_util.rs - utility functions for interacting with SpaceTraders
