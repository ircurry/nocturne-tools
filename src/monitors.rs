use std::process::{Command, Output};

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
    pub name: String,
    monitors: Vec<Monitor>,
}

impl Profile {
    pub fn new_profile1() -> Profile {
	Profile {
	    name: String::from("undocked"),
	    monitors: vec![
		Monitor {
		    name: MonitorName::Name("eDP-1".to_owned()),
		    resolution: MontiorResolution::Resolution {
			width: 2256,
			height: 1504,
			refresh_rate: 60,
		    },
		    position: MonitorPosition::Position {
			x: 0,
			y: 0,
		    },
		    scale: MonitorScale::Scale(2),
		    enabled: true,
		},
		Monitor {
		    name: MonitorName::Name(String::from("DP-2")),
		    resolution: MontiorResolution::Resolution {
			width: 1920,
			height: 1080,
			refresh_rate: 60,
		    },
		    position: MonitorPosition::Position {
			x: 0,
			y: 0,
		    },
		    scale: MonitorScale::Scale(1),
		    enabled: true,
		},
		Monitor {
		    name: MonitorName::All,
		    resolution: MontiorResolution::Prefered,
		    position: MonitorPosition::Auto,
		    scale: MonitorScale::Auto,
		    enabled: true,
		},
	    ],
	}
    }

    pub fn new_profile2() -> Profile {
	Profile {
	    name: String::from("docked"),
	    monitors: vec![
		Monitor {
		    name: MonitorName::Name("eDP-1".to_owned()),
		    resolution: MontiorResolution::Resolution {
			width: 2256,
			height: 1504,
			refresh_rate: 60,
		    },
		    position: MonitorPosition::Position {
			x: 0,
			y: 0,
		    },
		    scale: MonitorScale::Scale(2),
		    enabled: false,
		},
		Monitor {
		    name: MonitorName::Name(String::from("DP-2")),
		    resolution: MontiorResolution::Resolution {
			width: 1920,
			height: 1080,
			refresh_rate: 60,
		    },
		    position: MonitorPosition::Position {
			x: 0,
			y: 0,
		    },
		    scale: MonitorScale::Scale(1),
		    enabled: true,
		},
		Monitor {
		    name: MonitorName::All,
		    resolution: MontiorResolution::Prefered,
		    position: MonitorPosition::Auto,
		    scale: MonitorScale::Auto,
		    enabled: true,
		},
	    ],
	}
    }

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
		 .expect(format!("failed to execute command 'hyprctl keyword monitor {x}").as_str()))
	    .collect()
    }
}
