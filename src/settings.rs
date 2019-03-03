use serde::{Serialize, Deserialize};

#[derive(Clone, Default, Debug, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Config {
    pub graphics: Graphics,
}

#[derive(Clone, Default, Debug, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Graphics {
    pub scale: f32,
}

#[derive(Clone, Debug,)]
pub struct Logs {
    pub root: slog::Logger,
}

#[derive(Clone, Debug)]
pub struct Context {
    pub logs: Logs,
}
