use streaming::song::Song;
use streaming::Client;

use crate::stateful_list::StatefulList;
use crate::tab_state::TabsState;

pub struct App<'a> {
    pub search_results: StatefulList<Song>,
    pub queue: StatefulList<Song>,
    pub library: StatefulList<Song>,
    pub search_bar: String,
    pub tabs: TabsState<'a>,
}

impl<'a> App<'a> {
    pub async fn new(client: &mut Client) -> App<'a> {
        Self {
            search_results: StatefulList::with_items(Vec::new()),
            queue: StatefulList::with_items(Vec::new()),
            library: StatefulList::with_items(client.search_local("").await),
            search_bar: String::new(),
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
            0 => {
                if let Some(song) = self.search_results.get_selected() {
                    client.add_to_queue(song);
                }
            }
            1 => {
                if self.queue.get_selected().is_some() {
                    client.play_n(self.queue.state.selected().unwrap());
                }
            }
            2 => {
                if let Some(song) = self.library.get_selected() {
                    client.add_to_queue(song);
                }
            }
            _ => {}
        }
    }
}
