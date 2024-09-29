use serde::Deserialize;
use std::process::{Command, Output};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MonitorName {
    All,
    Name(String),
}

impl MonitorName {
    fn hyprland_string(&self) -> String {
        match self {
            MonitorName::All => String::new(),
            MonitorName::Name(name) => String::from(name),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MontiorResolution {
    Prefered,
    Resolution {
        width: i64,
        height: i64,
        refresh_rate: u8,
    },
}

impl MontiorResolution {
    fn hyprland_string(&self) -> String {
        match self {
            MontiorResolution::Prefered => String::from("prefered"),
            MontiorResolution::Resolution {
                width,
                height,
                refresh_rate,
            } => format!("{width}x{height}@{refresh_rate}"),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MonitorPosition {
    Auto,
    Position { x: i64, y: i64 },
}

impl MonitorPosition {
    fn hyprland_string(&self) -> String {
        match self {
            MonitorPosition::Auto => String::from("auto"),
            MonitorPosition::Position { x, y } => format!("{x}x{y}"),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MonitorScale {
    Auto,
    Scale(u8),
    Fractional(f32),
}

impl MonitorScale {
    fn hyprland_string(&self) -> String {
        match self {
            MonitorScale::Auto => String::from("auto"),
            MonitorScale::Scale(scale) => format!("{scale}"),
            MonitorScale::Fractional(scale) => format!("{scale}"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Monitor {
    name: MonitorName,
    resolution: MontiorResolution,
    position: MonitorPosition,
    scale: MonitorScale,
    enabled: bool,
}

impl Monitor {
    pub fn hyprland_string(&self) -> String {
        let name = self.name.hyprland_string();
        if !self.enabled {
            format!("{name},disabled")
        } else {
            let res = self.resolution.hyprland_string();
            let pos = self.position.hyprland_string();
            let scale = self.scale.hyprland_string();
            format!("{name},{res},{pos},{scale}")
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub name: String,
    monitors: Vec<Monitor>,
}

impl Profile {
    pub fn hyprland_strings(&self) -> Vec<String> {
        self.monitors.iter().map(|x| x.hyprland_string()).collect()
    }

    pub fn configure_monitors(&self) -> Vec<Output> {
        let mon_iter: Vec<String> = self.hyprland_strings();
        mon_iter
            .iter()
            .map(|ref x|
                 Command::new("hyprctl")
                 .args(["keyword", "monitor"])
                 .arg(x)
                 .output()
                 .expect(
                     format!("failed to execute command 'hyprctl keyword monitor {x}").as_str(),
                 )
            )
            .collect()
    }
}
