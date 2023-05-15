use spacedust::models::Agent;
use strum::IntoEnumIterator;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table, Tabs},
    Frame,
};

use crate::{
    app::{App, Tab},
    config::get_global_db_pool,
};

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // -------------------------------------------------------
    //                   Overall Layout
    // -------------------------------------------------------
    let screen = Layout::default()
        .vertical_margin(0)
        .horizontal_margin(1)
        .constraints([Constraint::Min(0)].as_ref())
        .split(frame.size())[0];
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .horizontal_margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(2)].as_ref())
        .split(screen);
    frame.render_widget(Block::default().borders(Borders::ALL), screen);

    // -------------------------------------------------------
    //                       Main Tabs
    // -------------------------------------------------------
    let tab_strs: Vec<String> = Tab::iter().map(|x| x.to_string()).collect();
    let tabs = tab_strs
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    frame.render_widget(
        Tabs::new(tabs)
            .select(Tab::iter().position(|x| x == app.tab).unwrap_or(0))
            .block(Block::default().title("Menu").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().bg(Color::White).fg(Color::Black))
            .divider(Span::raw("|")),
        chunks[0],
    );

    // -------------------------------------------------------
    //                    Individual Tabs
    // -------------------------------------------------------
    match app.tab {
        Tab::Agent => render_agent_tab(app, frame, chunks[1]),
        Tab::Systems => render_systems_tab(app, frame, chunks[1]),
        Tab::Fleet => render_fleet_tab(app, frame, chunks[1]),
    }
}

fn render_agent_tab<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, chunk: Rect) {
    let agent = sqlx::query_as!(Agent, "SELECT * FROM agents LIMIT 1")
        .fetch_one(get_global_db_pool().await)
        .await?;
}
fn render_systems_tab<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, chunk: Rect) {}
fn render_fleet_tab<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, chunk: Rect) {}
