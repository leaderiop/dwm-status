use crate::status::{Widget, WidgetMessage};
use std::sync::mpsc::Sender;

pub struct DateTime {
    format: String,
    status: String,
}

impl DateTime {
    pub fn new(format: &str) -> DateTime {
        DateTime {
            format: String::from(format),
            status: String::from(""),
        }
    }

    fn set_status(&mut self) {
        self.status.clear();
        let time = chrono::Local::now();
        self.status.push_str(&time.format(&self.format).to_string());
    }
}

impl Widget for DateTime {
    fn run(&mut self, sender: Sender<WidgetMessage>,index: usize) {
        loop {
            self.set_status();
            sender
                .send(WidgetMessage::Update {
                    index,
                    status: self.status.clone(),
                })
                .unwrap();
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }
}
