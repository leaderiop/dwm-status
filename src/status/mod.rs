mod battery;
mod date_time;
mod spacer;
mod wifi;
mod amixer;

pub use battery::Battery;
pub use date_time::DateTime;
pub use spacer::Spacer;
pub use wifi::Wifi;
pub use amixer::{Amixer, AmixerSource};

use crate::core::XDisplay;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

pub enum WidgetMessage {
    Update { index: usize, status: String },
}

pub trait Widget: Send {
    fn run(&mut self, _: Sender<WidgetMessage>, id: usize);
}

pub struct StatusBar {
    pub receiver: std::sync::mpsc::Receiver<WidgetMessage>,
    pub status: String,
    pub display: XDisplay,
    pub widgets: Vec<String>,
}

impl StatusBar {
    pub fn new(display: XDisplay, widgets: Vec<Box<dyn Widget>>) -> StatusBar {
        let widgets_vec: Vec<String> = vec!["".to_string(); widgets.len()];
        let (sender, receiver) = std::sync::mpsc::channel();
        for (index, widget) in widgets.into_iter().enumerate() {
            let widget = Arc::new(Mutex::new(widget));
            let sender = sender.clone();
            std::thread::spawn(move || {
                let mut widget = widget.lock().unwrap();
                widget.run(sender, index)
            });
        }

        let status = StatusBar {
            receiver,
            status: String::from(""),
            display,
            widgets: widgets_vec,
        };

        status
    }
    fn update_status(&mut self, index: usize, status: String) {
        self.widgets[index] = status;
        self.status = self.widgets.join("");
        self.display.set_name(&self.status);
    }

    pub fn run(&mut self) {
        loop {
            match self.receiver.recv() {
                Ok(message) => match message {
                    WidgetMessage::Update { index, status } => {
                        self.update_status(index, status);
                    }
                },
                Err(_) => {
                    continue;
                }
            }
        }
    }
}
