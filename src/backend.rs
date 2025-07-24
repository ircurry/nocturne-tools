use super::monitors::{
    Monitor, MonitorName, MonitorPosition, MonitorScale, MontiorResolution, Profile,
};
use clap::ValueEnum;
use std::process::{Command, Output};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Compositor {
    Hyprland,
}

fn hyprland_monitor_to_string(monitor: &Monitor) -> String {
    let name = match &monitor.name {
        MonitorName::All => String::new(),
        MonitorName::Name(name) => String::from(name),
    };
    if !monitor.enabled {
        format!("{name},disabled")
    } else {
        let res = match &monitor.resolution {
            MontiorResolution::Prefered => String::from("prefered"),
            MontiorResolution::Resolution {
                width,
                height,
                refresh_rate,
            } => format!("{width}x{height}@{refresh_rate}"),
        };
        let pos = match &monitor.position {
            MonitorPosition::Auto => String::from("auto"),
            MonitorPosition::Position { x, y } => format!("{x}x{y}"),
        };
        let scale = match &monitor.scale {
            MonitorScale::Auto => String::from("auto"),
            MonitorScale::Scale(scale) => format!("{scale}"),
            MonitorScale::Fractional(scale) => format!("{scale}"),
        };
        format!("{name},{res},{pos},{scale}")
    }
}

pub fn configure_monitors(compositor: &Compositor, profile: &Profile) -> Vec<Output> {
    match compositor {
        Compositor::Hyprland => profile
            .monitors
            .iter()
            .map(|mon| {
                let mon_str = hyprland_monitor_to_string(mon);
                Command::new("hyprctl")
                    .args(["keyword", "monitor"])
                    .arg(&mon_str)
                    .output()
                    .expect(
                        format!("failed to execute command \"hyprctl keyword monitor {mon_str}\"")
                            .as_str(),
                    )
            })
            .collect(),
    }
}
