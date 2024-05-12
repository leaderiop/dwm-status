use crate::status::{Widget, WidgetMessage};
use std::sync::mpsc::Sender;

#[derive(Clone)]
pub struct Spacer {
    value: String,
}
impl Spacer {
    pub fn new(value: impl Into<String>) -> Spacer {
        Spacer {
            value: value.into(),
        }
    }
}

impl Widget for Spacer {
    fn run(&mut self, sender: Sender<WidgetMessage>, index: usize) {
        sender
            .send(WidgetMessage::Update {
                index,
                status: self.value.clone(),
            })
            .unwrap();
    }
}
