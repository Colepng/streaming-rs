use streaming::song::Song;

use crate::stateful_list::StatefulList;
use crate::tab_state::TabsState;

pub struct App<'a> {
    pub queue: StatefulList<Song>,
    pub search_results: StatefulList<Song>,
    pub search_bar: String,
    pub tabs: TabsState<'a>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        Self {
            queue: StatefulList::with_items(Vec::new()),
            search_results: StatefulList::with_items(Vec::new()),
            search_bar: String::new(),
            tabs: TabsState::new(vec!["Search", "Queue"]),
        }
    }
}
