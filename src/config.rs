pub(crate) const GREEN: &str = "#08FF00";
pub(crate) const RED: &str = "#FF0000";
pub(crate) const YELLOW_WARNING: &str = "#FED037";
pub(crate) const YELLOW: &str = "#E5DE00";
pub(crate) const NEUTRAL: &str = "#FFFFFF";

pub struct Config;

impl Config {
    pub const fn new() -> Self {
        Config {}
    }

    pub fn get_wifi_device_name(&self) -> &str {
        "wlp3s0"
    }

    pub fn get_ethernet_device_name(&self) -> &str {
        "enp5s0"
    }

    pub fn battery_device_name(&self) -> &str {
        "BAT0"
    }

    pub fn brightness_device_name(&self) -> &str {
        "amdgpu_bl1"
    }

    pub fn widget_order(&self) -> Vec<String> {
        vec![
            String::from("wireless"),
            String::from("ethernet"),
            String::from("battery"),
            String::from("brightness"),
            String::from("cpu_load"),
            String::from("cpu_percentage"),
            String::from("memory"),
            String::from("disk"),
            String::from("time"),
        ]
    }

    pub fn timezone(&self) -> &str {
        "/etc/localtime"
    }
}
