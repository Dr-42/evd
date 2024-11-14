use chrono::TimeDelta;
use std::thread::sleep;

mod config;
mod eww;
mod volume;

fn main() -> Result<(), String> {
    let config = config::Config::load()?;
    let duration = TimeDelta::milliseconds(config.poll_time as i64);
    let mut volume_data = volume::VolumeData::new();
    let mut notif_timer = TimeDelta::milliseconds(config.notification_fade_time as i64);
    loop {
        if volume_data.changed {
            notif_timer = TimeDelta::milliseconds(config.notification_fade_time as i64);
            eww::set(&config, &volume_data);
            eww::show(&config);
        }
        volume_data.get();
        notif_timer -= duration;
        if notif_timer < TimeDelta::milliseconds(0) {
            eww::close(&config);
        }
        sleep(duration.to_std().unwrap());
    }
}
