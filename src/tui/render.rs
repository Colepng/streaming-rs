use ratatui::{
    prelude::{Backend, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};

use crate::app::App;

pub fn render<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let tabs = app.tabs.titles.iter().map(|x| Line::from(*x)).collect::<Vec<Line>>();

    let tabs = Tabs::new(tabs)
        .block(Block::default().borders(Borders::ALL)).highlight_style(Style::default().bold()).select(app.tabs.index);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Max(3),
            Constraint::Min(1),
        ])
        .split(frame.size());

    frame.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => render_search(frame, app, chunks[1]),
        1 => render_queue(frame, app, chunks[1]),
        _ => {},
    }
}

pub fn render_search<B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default().direction(Direction::Vertical).constraints(vec![Constraint::Max(3), Constraint::Min(1)]).split(area);
    let text =
        Paragraph::new(app.search_bar.as_str()).block(Block::default().borders(Borders::ALL));

    let songs: Vec<ListItem> = app
        .search_results
        .items
        .iter()
        .map(|song| ListItem::new::<String>(format!("{} by {}", song.name, song.artist)))
        .collect();
    let list = List::new(songs)
        .block(Block::default().title("Songs").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>");
    frame.render_widget(text, chunks[0]);
    frame.render_stateful_widget(list, chunks[1], &mut app.search_results.state);
}

pub fn render_queue<B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) {
    let songs: Vec<ListItem> = app
        .queue
        .items
        .iter()
        .map(|song| ListItem::new::<String>(format!("{} by {}", song.name, song.artist)))
        .collect();

    let list = List::new(songs)
        .block(Block::default().title("Songs").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>");

    frame.render_stateful_widget(list, area, &mut app.queue.state)
}
