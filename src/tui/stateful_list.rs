use ratatui::widgets::ListState;

// https://github.com/ratatui-org/ratatui/blob/main/examples/list.rs
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::with_selected(ListState::default(), Some(0)),
            items,
        }
    }

    pub fn next(&mut self) {
        if !self.items.is_empty() {
            let i = match self.state.selected() {
                Some(i) => {
                    if i >= self.items.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.state.select(Some(i));
        }
    }

    pub fn previous(&mut self) {
        if !self.items.is_empty() {
            let i = match self.state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.items.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.state.select(Some(i));
        }
    }

    // fn unselect(&mut self) {
    //     self.state.select(None);
    // }

    pub fn get_selected(&mut self) -> Option<&T> {
        if let Some(pos) = self.state.selected() {
            Some(&self.items[pos])
        } else {
            None
        }
    }
}
