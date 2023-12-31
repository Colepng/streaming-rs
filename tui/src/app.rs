use crossterm::event::KeyCode;
use streaming::song::Song;
use streaming::Client;

use crate::stateful_list::StatefulList;
use crate::tab_state::TabsState;

pub struct App<'a> {
    pub search_results: StatefulList<Song>,
    pub queue: StatefulList<Song>,
    pub library: StatefulList<Song>,
    pub search_bar: String,
    pub search_future: Option<tokio::task::JoinHandle<Result<Vec<Song>, streaming::SearchError>>>,
    pub tabs: TabsState<'a>,
}

impl<'a> App<'a> {
    pub async fn new(client: &mut Client) -> App<'a> {
        Self {
            search_results: StatefulList::with_items(Vec::new()),
            queue: StatefulList::with_items(Vec::new()),
            library: StatefulList::with_items(client.search_local("").await),
            search_bar: String::new(),
            search_future: None,
            tabs: TabsState::new(vec!["Search", "Queue", "Library"]),
        }
    }

    pub fn next_item(&mut self) {
        match self.tabs.index {
            0 => self.search_results.next(),
            1 => self.queue.next(),
            2 => self.library.next(),
            _ => {}
        }
    }

    pub fn previous_item(&mut self) {
        match self.tabs.index {
            0 => self.search_results.previous(),
            1 => self.queue.previous(),
            2 => self.library.previous(),
            _ => {}
        }
    }

    pub fn select(&mut self, client: &mut Client) {
        match self.tabs.index {
            // search_results
            0 => {
                if let Some(song) = self.search_results.get_selected() {
                    client.add_to_queue(song);
                }
            }
            // queue
            1 => {
                if self.queue.state.selected().is_some() {
                    client.play_n(self.queue.state.selected().unwrap());
                }
            }
            // library
            2 => {
                if let Some(song) = self.library.get_selected() {
                    client.add_to_queue(song);
                }
            }
            _ => {}
        }
    }

    pub async fn handle_tabs_input(&mut self, client: &mut Client, input: KeyCode) {
        match self.tabs.index {
            0 => self.handle_search_tab_input(input),
            1 => self.handle_queue_tab_input(input, client).await,
            2 => self.handle_library_tab_input(input, client).await,
            _ => {}
        }
    }

    fn handle_search_tab_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(char) => {
                self.search_bar.push(char);
                let search_bar = self.search_bar.clone();
                self.search_future = Some(tokio::spawn(async move {
                    let songs = streaming::search(search_bar.clone()).await;
                    songs
                }));
            }
            KeyCode::Backspace => {
                self.search_bar.pop();
                let search_bar = self.search_bar.clone();
                if !search_bar.is_empty() {
                    self.search_future = Some(tokio::spawn(async move {
                        let songs = streaming::search(search_bar.clone()).await;
                        songs
                    }));
                }
            }
            _ => {}
        }
    }

    async fn handle_queue_tab_input(&mut self, key: KeyCode, client: &mut Client) {
        match key {
            KeyCode::Char(char) => match char {
                ' ' => client.toggle(),
                'd' => {
                    client.remove_from_queue(self.queue.index().unwrap());
                    self.queue.items = client.get_songs();
                    if self.queue.is_out_of_bounds() {
                        self.queue.fix_out_of_bounds();
                    }
                }
                'a' => {
                    if let Some(song) = self.queue.get_selected() {
                        client.add_song(&song).await;
                        self.library.items = client.search_local("").await;
                        if self.library.is_out_of_bounds() {
                            self.library.fix_out_of_bounds();
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    async fn handle_library_tab_input(&mut self, key: KeyCode, client: &mut Client) {
        match key {
            KeyCode::Char(char) => match char {
                ' ' => client.toggle(),
                'd' => {
                    if let Some(song) = self.library.get_selected() {
                        let song = song.clone();
                        let client = client.clone();
                        tokio::spawn(async move {
                            let _ = client.download(&song).await;
                        });
                    }
                }
                'D' => {
                    let _ = client.delete(self.library.get_selected().unwrap());
                }
                'r' => {
                    if let Some(song) = self.library.get_selected() {
                        if song.is_downloaded() && client.delete(song).is_ok() {
                            client.remove_song(song).await;
                            self.library.items = client.search_local("").await;
                            if self.library.is_out_of_bounds() {
                                self.library.fix_out_of_bounds();
                            }
                        } else if !song.is_downloaded() {
                            client.remove_song(song).await;
                            self.library.items = client.search_local("").await;
                            if self.library.is_out_of_bounds() {
                                self.library.fix_out_of_bounds();
                            }
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}
