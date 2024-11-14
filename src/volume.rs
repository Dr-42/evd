pub struct VolumeData {
    pub volume: f32,
    pub volume_string: String,
    pub changed: bool,
}

impl VolumeData {
    pub fn new() -> VolumeData {
        VolumeData {
            volume: 0.0,
            volume_string: String::from(""),
            changed: true,
        }
    }

    pub fn get(&mut self) {
        let volume = std::process::Command::new("pactl")
            .arg("get-sink-volume")
            .arg("@DEFAULT_SINK@")
            .output()
            .unwrap();
        let volume_string = String::from_utf8(volume.stdout).unwrap();
        let volume_string = volume_string
            .split("Volume: ")
            .nth(1)
            .unwrap()
            .split(" / ")
            .nth(1)
            .unwrap();
        let volume = volume_string.split("%").nth(0).unwrap();
        let volume = volume.trim().parse::<f32>().unwrap();
        let changed = (volume - self.volume).abs() > 1.0;
        self.volume = volume;
        self.volume_string = volume_string.to_string();
        self.changed = changed;
    }
}
