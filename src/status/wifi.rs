use crate::status::{Widget, WidgetMessage};
use crate::utils::spawn_for_output;
use std::fs;
use std::sync::mpsc::Sender;

pub struct Wifi {
    pub status: String,
}

impl Wifi {
    pub fn new() -> Wifi {
        Wifi {
            status: String::from(""),
        }
    }

    fn wifi_text(&self) -> Option<String> {
        let (interface, _) = self.interface_and_essid()?;
        let signal = self.signal_quality(&interface)?;

        Some(format!("  {signal}%"))
    }

    // Read the interface name and essid via iwgetid.
    //   Output format is '$interface    ESSID:"$essid"'
    fn interface_and_essid(&self) -> Option<(String, String)> {
        let raw = spawn_for_output("iwgetid").ok()?;
        let mut iter = raw.split(':');

        // Not using split_whitespace here as the essid may contain whitespace
        let interface = iter.next()?.split_whitespace().next()?.to_owned();
        let essid = iter.next()?.split('"').nth(1)?.to_string();

        Some((interface, essid))
    }

    // Parsing the format described here: https://hewlettpackard.github.io/wireless-tools/Linux.Wireless.Extensions.html
    fn signal_quality(&self, interface: &str) -> Option<String> {
        let raw = fs::read_to_string("/proc/net/wireless").ok()?;

        for line in raw.lines() {
            if line.starts_with(interface) {
                return Some(
                    line.split_whitespace()
                        .nth(2)?
                        .strip_suffix('.')?
                        .to_owned(),
                );
            }
        }

        None
    }
}

impl Widget for Wifi {
    fn run(&mut self, sender: Sender<WidgetMessage>, index: usize) {
        loop {
            if let Some(text) = self.wifi_text() {
                sender
                    .send(WidgetMessage::Update {
                        index,
                        status: text,
                    })
                    .unwrap();
            }
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    }
}
