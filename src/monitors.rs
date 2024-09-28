#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum MonitorScale {
    Auto,
    Scale(u8),
}

impl MonitorScale {
    fn hyprland_string(&self) -> String {
        match self {
            MonitorScale::Auto => String::from("auto"),
            MonitorScale::Scale(scale) => format!("{scale}"),
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Profile {
    name: String,
    monitors: Vec<Monitor>,
    default_monitor: Monitor,
}
