use crate::status::{Widget, WidgetMessage};
use std::fs;
use std::sync::mpsc::Sender;
pub struct Battery {
    status: String,
    name: String,
}

impl Battery {
    pub fn new(name: impl Into<String>) -> Battery {
        Battery {
            status: String::from(""),
            name: name.into(),
        }
    }

    fn battery_text(&self, bat: &str) -> Option<String> {
        let status = self.read_sys_file(bat, "status")?;
        let energy_now: u32 = self.read_sys_file(bat, "energy_now")?.parse().ok()?;
        let energy_full: u32 = self.read_sys_file(bat, "energy_full")?.parse().ok()?;

        let charge = (energy_now / energy_full) * 100;

        let icon = if status == "Charging" {
            ""
        } else if charge >= 90 || status == "Full" {
            ""
        } else if charge >= 70 {
            ""
        } else if charge >= 50 {
            ""
        } else if charge >= 20 {
            ""
        } else {
            ""
        };

        Some(format!("{icon} {charge}%"))
    }

    fn read_sys_file(&self, bat: &str, fname: &str) -> Option<String> {
        fs::read_to_string(format!("/sys/class/power_supply/{bat}/{fname}"))
            .ok()
            .map(|s| s.trim().to_string())
    }
}
impl Widget for Battery {
    fn run(&mut self, sender: Sender<WidgetMessage>,index: usize) {
        loop {
            if let Some(status) = self.battery_text(self.name.as_str()) {
                self.status = status;
                sender
                    .send(WidgetMessage::Update {
                        index,
                        status: self.status.clone(),
                    })
                    .unwrap();
            }
            std::thread::sleep(std::time::Duration::from_secs(10));
        }
    }

}
