use once_cell::sync::Lazy;
use strum::IntoEnumIterator;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Row, Table, Tabs},
    Frame,
};

use crate::{
    app::{App, Tab},
    st_util,
};

static BASE_STYLE: Lazy<Style> = Lazy::new(|| Style::default().fg(Color::White));
static TITLE_STYLE: Lazy<Style> = Lazy::new(|| *BASE_STYLE);
static VALUE_STYLE: Lazy<Style> = Lazy::new(|| *BASE_STYLE);
static KEY_STYLE: Lazy<Style> = Lazy::new(|| BASE_STYLE.add_modifier(Modifier::BOLD));
static HEADER_STYLE: Lazy<Style> = Lazy::new(|| *KEY_STYLE);
static TAB_STYLE: Lazy<Style> = Lazy::new(|| *BASE_STYLE);
static TAB_SELECTED_STYLE: Lazy<Style> = Lazy::new(|| {
    TAB_STYLE
        .fg(TAB_STYLE.bg.unwrap_or(Color::Black))
        .bg(TAB_STYLE.fg.unwrap_or(Color::White))
});
static LIST_STYLE: Lazy<Style> = Lazy::new(|| *BASE_STYLE);
static LIST_SELECTED_STYLE: Lazy<Style> = Lazy::new(|| {
    LIST_STYLE
        .fg(LIST_STYLE.bg.unwrap_or(Color::Black))
        .bg(LIST_STYLE.fg.unwrap_or(Color::White))
});

static BASE_BLOCK: Lazy<Block> =
    Lazy::new(|| Block::default().style(*TITLE_STYLE).borders(Borders::ALL));

macro_rules! key_value {
    ($key:expr, $val:expr) => {
        Spans::from(vec![
            Span::styled(format!(" {}: ", $key), *KEY_STYLE),
            Span::styled($val, *VALUE_STYLE),
        ])
    };
}

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
    frame.render_widget(BASE_BLOCK.clone(), screen);

    // -------------------------------------------------------
    //                       Main Tabs
    // -------------------------------------------------------
    let tab_strs: Vec<String> = Tab::iter().map(|x| x.to_string()).collect();
    let tabs = tab_strs
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, TAB_STYLE.add_modifier(Modifier::UNDERLINED)),
                Span::styled(rest, *TAB_STYLE),
            ])
        })
        .collect();

    frame.render_widget(
        Tabs::new(tabs)
            .select(Tab::iter().position(|x| x == app.state.tab).unwrap_or(0))
            .block(BASE_BLOCK.clone().title("Menu"))
            .style(*TAB_STYLE)
            .highlight_style(*TAB_SELECTED_STYLE)
            .divider(Span::raw("|")),
        chunks[0],
    );

    // -------------------------------------------------------
    //                    Individual Tabs
    // -------------------------------------------------------
    match app.state.tab {
        Tab::Agent => render_agent_tab(app, frame, chunks[1]),
        Tab::Systems => render_systems_tab(app, frame, chunks[1]),
        Tab::Fleet => render_fleet_tab(app, frame, chunks[1]),
    }
}

fn render_agent_tab<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, chunk: Rect) {
    #[allow(clippy::cast_possible_truncation)]
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(5),
                Constraint::Percentage(100),
                Constraint::Min(app.state.factions.len() as u16 + 3), // +3 for border + table header
            ]
            .as_ref(),
        )
        .split(chunk);

    render_agent_block(app, frame, chunks[0]);
    render_contracts_block(app, frame, chunks[1]);
    render_factions_block(app, frame, chunks[2]);
}

fn render_agent_block<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, chunk: Rect) {
    let agent = &app.state.agent;

    let agent_info = Paragraph::new(vec![
        key_value!("Symbol", &agent.symbol),
        key_value!("Headquarters", &agent.headquarters),
        key_value!("Credits", agent.credits.to_string()),
    ])
    .block(BASE_BLOCK.clone().title("Me"));

    frame.render_widget(agent_info, chunk);
}

fn render_contracts_block<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, chunk: Rect) {
    let contracts = &app.state.contracts;

    #[allow(clippy::cast_possible_truncation)]
    let id_max_length = contracts
        .iter()
        .fold(10, |id_max, c| id_max.max(c.id.len() as u16));

    let contracts_info_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Max(id_max_length), Constraint::Min(30)].as_ref())
        .margin(1)
        .split(chunk);

    let contracts_block = BASE_BLOCK.clone().title("Contracts");
    frame.render_widget(contracts_block, chunk);

    let contracts_list_items: Vec<ListItem> = contracts
        .iter()
        .map(|c| ListItem::new(c.id.clone()))
        .collect();
    let contracts_list = List::new(contracts_list_items)
        .style(*LIST_STYLE)
        .highlight_style(*LIST_SELECTED_STYLE)
        .repeat_highlight_symbol(true)
        .block(BASE_BLOCK.clone().borders(Borders::RIGHT));

    frame.render_stateful_widget(
        contracts_list,
        contracts_info_chunks[0],
        &mut app.state.contracts_list_state,
    );

    if let Some(index) = app.state.contracts_list_state.selected() {
        let selected_contract = &contracts[index];

        let contract_details_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(8), Constraint::Percentage(100)].as_ref())
            .split(contracts_info_chunks[1]);

        let contract_details = Paragraph::new(vec![
            key_value!("Faction", &selected_contract.faction_symbol),
            key_value!(
                "Type",
                st_util::contract_type_to_string(&selected_contract.r#type)
            ),
            key_value!("Accepted", selected_contract.accepted.to_string()),
            key_value!("Fulfilled", selected_contract.fulfilled.to_string()),
            key_value!("Deadline", &selected_contract.terms.deadline),
            key_value!("Expiration", &selected_contract.expiration),
            key_value!(
                "Initial Payment",
                selected_contract.terms.payment.on_accepted.to_string()
            ),
            key_value!(
                "Fulfillment Payment",
                selected_contract.terms.payment.on_fulfilled.to_string()
            ),
        ]);

        if let Some(delivers) = &selected_contract.terms.deliver {
            let contract_deliver_rows = delivers.iter().map(|d| {
                Row::new(vec![
                    d.trade_symbol.clone(),
                    d.destination_symbol.clone(),
                    d.units_required.to_string(),
                    d.units_fulfilled.to_string(),
                ])
                .style(*BASE_STYLE)
            });
            let contract_deliver = Table::new(contract_deliver_rows)
                .header(
                    Row::new(vec![
                        "Good",
                        "Destination",
                        "Units Required",
                        "Units Fulfilled",
                    ])
                    .style(*HEADER_STYLE),
                )
                .widths([Constraint::Percentage(25); 25].as_ref())
                .column_spacing(2)
                .block(BASE_BLOCK.clone().borders(Borders::TOP));
            frame.render_widget(contract_deliver, contract_details_chunks[1]);
        }

        frame.render_widget(contract_details, contract_details_chunks[0]);
    }
}

fn render_factions_block<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, chunk: Rect) {
    let factions = &app.state.factions;

    // Calculate maximum lengths of different columns
    #[allow(clippy::cast_possible_truncation)]
    let (symbol_max_length, name_max_length, headquarters_max_length): (u16, u16, u16) =
        factions.iter().fold(
            (
                "Symbol".len() as u16,
                "Name".len() as u16,
                "Headquarters".len() as u16,
            ),
            |(s_max, n_max, h_max), f| {
                (
                    s_max.max(f.symbol.len() as u16),
                    n_max.max(f.name.len() as u16),
                    h_max.max(f.headquarters.len() as u16),
                )
            },
        );

    let factions_info_rows = factions.iter().map(|f| {
        Row::new(vec![
            f.symbol.clone(),
            f.name.clone(),
            f.headquarters.clone(),
            f.description.clone(),
        ])
        .style(*BASE_STYLE)
    });
    let factions_info_widths = [
        Constraint::Length(symbol_max_length),
        Constraint::Length(name_max_length),
        Constraint::Length(headquarters_max_length),
        Constraint::Percentage(100),
    ];

    let factions_info = Table::new(factions_info_rows)
        .header(
            Row::new(vec!["Symbol", "Name", "Headquarters", "Description"]).style(*HEADER_STYLE),
        )
        .widths(factions_info_widths.as_ref())
        .column_spacing(2)
        .block(BASE_BLOCK.clone().title("Factions"));

    frame.render_widget(factions_info, chunk);
}

fn render_systems_tab<B: Backend>(_app: &mut App, _frame: &mut Frame<'_, B>, _chunk: Rect) {}
fn render_fleet_tab<B: Backend>(_app: &mut App, _frame: &mut Frame<'_, B>, _chunk: Rect) {}
