use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MonitorName {
    All,
    Name(String),
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

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MonitorPosition {
    Auto,
    Position { x: i64, y: i64 },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MonitorScale {
    Auto,
    Scale(u8),
    Fractional(f32),
}

#[derive(Debug, Deserialize)]
pub struct Monitor {
    pub name: MonitorName,
    pub resolution: MontiorResolution,
    pub position: MonitorPosition,
    pub scale: MonitorScale,
    pub enabled: bool,
}

impl Monitor {
    pub fn to_string(&self) -> String {
        let name = match &self.name {
            MonitorName::All => "Default Monitor Settings:".to_string(),
            MonitorName::Name(string) => format!("Monitor \"{string}\":"),
        };
        let res = match &self.resolution {
            MontiorResolution::Prefered => "prefered".to_string(),
            MontiorResolution::Resolution {
                width,
                height,
                refresh_rate,
            } => format!("{width}x{height}@{refresh_rate}"),
        };
        let scale = match &self.scale {
            MonitorScale::Auto => "auto".to_string(),
            MonitorScale::Scale(scale) => scale.to_string(),
            MonitorScale::Fractional(scale) => scale.to_string(),
        };
        let pos = match &self.position {
            MonitorPosition::Auto => "auto".to_string(),
            MonitorPosition::Position { x, y } => format!("x={x}, y={y}"),
        };
        let enabled = &self.enabled.to_string();
        format!(
            r#"
{name}
  Resolution: {res}
  Position: {pos}
  Scale: {scale}
  Enabled: {enabled}"#
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub name: String,
    pub monitors: Vec<Monitor>,
}
