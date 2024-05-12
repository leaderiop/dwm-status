use dwm_status::{
    core::XDisplay,
    status::{Amixer,AmixerSource,Battery, DateTime, Spacer, StatusBar, Wifi},
};
fn main() {
    let display = XDisplay::new();
    let spacer = Spacer::new(" | ");
    let mut statusbar = StatusBar::new(
        display,
        vec![
            Box::new(Amixer::new("Capture", AmixerSource::Input)),
            Box::new(spacer.clone()),
            Box::new(Amixer::new("Master", AmixerSource::Output)),
            Box::new(spacer.clone()),
            Box::new(Battery::new("BAT0")),
            Box::new(spacer.clone()),
            Box::new(Wifi::new()),
            Box::new(spacer.clone()),
            Box::new(DateTime::new("%H:%M:%S")),
        ],
    );
    statusbar.run();
}
