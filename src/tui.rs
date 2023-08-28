use std::time::Duration;

use ratatui::{prelude::{CrosstermBackend, Rect}, Terminal, widgets::{Block, Borders, Paragraph, ListItem, List, ListState}, style::{Style, Color, Modifier}};
use crossterm::{event::{self, EnableMouseCapture, DisableMouseCapture, Event}, terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, execute}; 
use crossterm::terminal::enable_raw_mode;
use crossterm::event::KeyCode;
use rodio::OutputStream;
use streaming::{Client, song::Song};

fn main() -> Result<(), Box<std::io::Error>> {
    let mut stdout = std::io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let mut client = Client::new(stream_handle);

    client.init();

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("this is a title").borders(Borders::ALL);

        f.render_widget(block, size);
    })?;

    let mut input = String::new();
    let mut songs: Vec<Song> = Vec::new();
    let mut list_items: Vec<ListItem> = Vec::new();
    let mut list_state = ListState::default();
    let mut pos: usize = 0;
    loop {
        terminal.draw(|frame| {
            let text = Paragraph::new(input.as_str()).block(Block::default().borders(Borders::ALL));

            list_items.clear();
            for song in &songs {
                let name = format!("{} by {}", song.name, song.artist);
                list_items.push(ListItem::new(name));
            }

            let list = List::new(list_items.clone())
                .block(Block::default().title("Songs").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">>");

            frame.render_widget(text, Rect::new(0, 0, frame.size().width, 3));
            frame.render_stateful_widget(list, Rect::new(0, 3, frame.size().width, frame.size().height-3), &mut list_state);
        })?;
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Esc == key.code {
                    break;
                } else if let KeyCode::Char(key) = key.code {
                    input.push(key);
                    songs = streaming::search(input.as_str());
                } else if KeyCode::Backspace == key.code {
                    input.pop();
                    if !input.is_empty() {
                        songs = streaming::search(input.as_str());
                    }
                } else if KeyCode::Down == key.code {
                    if pos != songs.len() - 1{
                        pos += 1;
                        list_state.select(Some(pos));
                    }
                } else if KeyCode::Up == key.code {
                    if pos != 0 {
                        pos -= 1;
                        list_state.select(Some(pos));
                    }
                } else if KeyCode::Enter == key.code {
                    client.add_to_queue(songs[pos].clone());
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}
