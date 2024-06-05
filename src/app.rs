use ratatui::widgets::ListState;

pub struct StatefulPIDList {
    pub state: ListState,
    items: Vec<String>,
    last_selected: Option<usize>,
}

pub struct StatefulLabList {
    pub state: ListState,
    items: Vec<String>,
    last_selected: Option<usize>,
}

pub struct Result {
    sid: String,
    _tid: String,
    _pid: String,
    _result_json: String,
}

pub struct App {
    pub pid_list: StatefulPIDList,
    pub lab_list: StatefulLabList,
}

impl App {
    pub fn new() -> App {
        let pids = Self::get_pid_list();
        if pids.is_empty() {
            panic!("PID list is empty. This should never happen");
        }

        let labs = Self::get_labs_list();

        App {
            pid_list: StatefulPIDList::with_items(pids),
            lab_list: StatefulLabList::with_items(labs),
        }
    }

    pub fn get_pid_items(&self) -> Vec<String> {
        self.pid_list.items.clone()
    }

    pub fn next_patient(&mut self) {
        self.pid_list.next()
    }

    pub fn previous_patient(&mut self) {
        self.pid_list.previous()
    }

    pub fn next_lab(&mut self) {
        self.lab_list.next()
    }

    pub fn previous_lab(&mut self) {
        self.lab_list.previous()
    }

    pub fn get_lab_items(&self) -> Vec<String> {
        self.lab_list.items.clone()
    }

    pub fn get_selected_lab(&self) -> String {
        let i = self.lab_list.state.selected().unwrap_or(0);
        self.lab_list.items.get(i).unwrap().to_string()
    }

    pub fn get_selected_pid(&self) -> String {
        let i = self.pid_list.state.selected().unwrap_or(0);
        self.pid_list.items.get(i).unwrap().to_string()
    }

    fn get_pid_list() -> Vec<String> {
        vec![
            "1234567".to_string(),
            "9876544".to_string(),
            "3456678".to_string(),
        ]
    }

    fn get_labs_list() -> Vec<String> {
        vec![
            "12345678".to_string(),
            "98765448".to_string(),
            "34566788".to_string(),
        ]
    }
}

impl StatefulPIDList {
    fn with_items(items: Vec<String>) -> StatefulPIDList {
        StatefulPIDList {
            state: ListState::default().with_selected(Some(0)),
            items: items.iter().map(|x| x.clone()).collect(),
            last_selected: None,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }
}

impl StatefulLabList {
    fn with_items(items: Vec<String>) -> StatefulLabList {
        StatefulLabList {
            state: ListState::default().with_selected(Some(0)),
            items: items.iter().map(|x| x.clone()).collect(),
            last_selected: None,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }
}
