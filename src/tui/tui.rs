use std::time::Duration;

use app::App;
use ratatui::{prelude::CrosstermBackend, Terminal};
use crossterm::{event::{self, EnableMouseCapture, DisableMouseCapture, Event, KeyEventKind}, terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, execute}; 
use crossterm::terminal::enable_raw_mode;
use crossterm::event::KeyCode;
use rodio::OutputStream;
use streaming::Client;

mod render;
mod stateful_list;
mod app;

use render::render;
use tokio::sync::mpsc;
#[tokio::main]
async fn main() -> Result<(), Box<std::io::Error>> {
    let mut stdout = std::io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let mut client = Client::new(stream_handle).await;

    client.init();

    let mut app = App::new();

    let mut search_results = None;

    loop {
        terminal.draw(|frame| { render(frame, &mut app); })?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(key) => {
                            app.search_bar.push(key);
                            let search_bar = app.search_bar.clone();
                            search_results = Some(tokio::spawn(async move {
                                let songs = streaming::search(search_bar.clone()).await;
                                songs
                            }));
                        }
                        KeyCode::Backspace => {
                            app.search_bar.pop();
                            let search_bar = app.search_bar.clone();
                            if !search_bar.is_empty() {
                                search_results = Some(tokio::spawn(async move {
                                    // if !search_bar.is_empty() {
                                    let songs = streaming::search(search_bar.clone()).await;
                                    songs
                                        // }
                                }));
                            }
                        }
                        KeyCode::Down => {
                            app.queue.next();
                        }
                        KeyCode::Up => {
                            app.queue.previous();
                        }
                        KeyCode::Enter => {
                            if let Some(song) = app.queue.get_selected() {
                                client.add_to_queue(song);
                            }
                        }
                        KeyCode::Esc => {
                            break;
                        }
                        _ => {}
                    }

                }
            }

        }
        if let Some(ref handle) = search_results {
            if handle.is_finished() {
                app.queue.items = search_results.unwrap().await.unwrap();
                search_results = None
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}
