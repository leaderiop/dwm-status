use crate::status::{Widget, WidgetMessage};
use crate::utils::spawn_for_output;
use std::sync::mpsc::Sender;
pub enum AmixerSource {
    Output,
    Input,
}
pub struct Amixer {
    channel: String,
    source: AmixerSource,
}

impl Amixer {
    pub fn new(channel: impl Into<String>,source: AmixerSource) -> Amixer {
        Amixer {
            channel: channel.into(),
            source
        }
    }
    fn amixer_output_icon(&self, volume: i32, on: bool) -> String {
        if !on {
            return "".to_string();
        }
        match volume {
            0 => "",
            1..=50 => "",
            _ => "",
        }
        .to_string()
    }
    fn amixer_input_icon(&self, volume: i32, on: bool) -> String {
        if !on {
            return "".to_string();
        }
        match volume {
            0 => "",
            _ => "",
        }
        .to_string()
    }
    fn amixer_icon(&self, volume: i32, on: bool) -> String {
        match self.source {
            AmixerSource::Output => self.amixer_output_icon(volume, on),
            AmixerSource::Input => self.amixer_input_icon(volume, on),
        }
    }


    fn amixer_text(&self, channel: impl Into<String>) -> Option<String> {
        let raw = spawn_for_output(format!("amixer sget {}", channel.into())).ok()?;
        let raw = raw.lines().find(|line| line.contains("Front Left:"))?;
        let volume = raw.split_whitespace().nth(4)?
            .replace(|c| "[]%".contains(c), "").parse::<i32>().ok()?;
        let on = raw.split_whitespace().nth(5)? == "[on]";
        let icon = self.amixer_icon(volume, on);
        Some(format!("{icon} {volume}%"))
    }
}
impl Widget for Amixer {
    fn run(&mut self, sender: Sender<WidgetMessage>, index: usize) {
        loop {
            if let Some(status) = self.amixer_text(self.channel.clone()) {
                sender
                    .send(WidgetMessage::Update {
                        index,
                        status,
                    })
                    .unwrap();
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
