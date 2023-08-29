use streaming::song::Song;

use crate::stateful_list::StatefulList;

pub struct App {
    pub queue: StatefulList<Song>,
    pub search_bar: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            queue: StatefulList::with_items(Vec::new()),
            search_bar: String::new(),
        }
    }
}
