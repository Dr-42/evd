use crate::{config::Config, volume::VolumeData};

pub fn set(config: &Config, volume_data: &VolumeData) {
    // Commands : eww update <value_name>=<value>
    std::process::Command::new(&config.eww_binary_path)
        .arg("update")
        .arg(format!(
            "{}={:.2}",
            &config.eww_value_name, volume_data.volume
        ))
        .output()
        .unwrap();
    std::process::Command::new(&config.eww_binary_path)
        .arg("update")
        .arg(format!(
            "{}={}",
            &config.eww_value_string_name, &volume_data.volume_string
        ))
        .output()
        .unwrap();
}

fn is_window_open(cfg: &Config, window: &str) -> bool {
    let mut cmd = String::new();
    cmd.push_str(&cfg.eww_binary_path);
    cmd.push_str(" active-windows");
    //println!("{}", cmd);
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .expect("Failed to execute command");
    let output = String::from_utf8_lossy(&output.stdout);
    output.contains(format!("{}: {}\n", window, window).as_str())
}

pub fn show(config: &Config) {
    if !is_window_open(config, &config.eww_window_name) {
        std::process::Command::new(&config.eww_binary_path)
            .arg("open")
            .arg(&config.eww_window_name)
            .output()
            .unwrap();
    }
}

pub fn close(config: &Config) {
    if is_window_open(config, &config.eww_window_name) {
        std::process::Command::new(&config.eww_binary_path)
            .arg("close")
            .arg(&config.eww_window_name)
            .output()
            .unwrap();
    }
}
