use std::time::Duration;

use app::App;
use crossterm::event::KeyCode;
use crossterm::terminal::enable_raw_mode;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use rodio::OutputStream;
use streaming::Client;

mod app;
mod render;
mod stateful_list;
mod tab_state;

use render::render;
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

    let mut app = App::new(&mut client).await;

    loop {
        terminal.draw(|frame| {
            render(frame, &mut app, &mut client);
        })?;
        app.queue.items = client.get_songs();
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::F(n) => {
                            if n as usize <= app.tabs.titles.len() {
                                app.tabs.index = n as usize - 1;
                            }
                        }
                        KeyCode::Down => {
                            app.next_item();
                        }
                        KeyCode::Up => {
                            app.previous_item();
                        }
                        KeyCode::Enter => {
                            app.select(&mut client);
                        }
                        KeyCode::Esc => {
                            break;
                        }
                        KeyCode::Tab => {
                            app.tabs.next();
                        }
                        key => {
                            app.handle_tabs_input(&mut client, key);
                        }
                    }
                }
            }
        }
        if let Some(ref handle) = app.search_future {
            if handle.is_finished() {
                app.search_results.items = app.search_future.unwrap().await.unwrap();
                app.search_future = None;
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
