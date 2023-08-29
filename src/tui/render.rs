use ratatui::{prelude::{Backend, Rect}, Frame, widgets::{Paragraph, ListItem, Block, List, Borders}, style::{Color, Style, Modifier}};

use crate::app::App;

pub fn render<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let text = Paragraph::new(app.search_bar.as_str()).block(Block::default().borders(Borders::ALL));

    let songs: Vec<ListItem> = app.queue.items.iter().map(|song| ListItem::new::<&str>(&song.name)).collect();
    let list = List::new(songs)
        .block(Block::default().title("Songs").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>");

    frame.render_widget(text, Rect::new(0, 0, frame.size().width, 3));
    frame.render_stateful_widget(list, Rect::new(0, 3, frame.size().width, frame.size().height-3), &mut app.queue.state);
}
