use serde::{Serialize, Deserialize};

#[derive(Clone, Default, Debug, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct GameSettings {
    pub graphics: GraphicsSettings,
}

#[derive(Clone, Default, Debug, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct GraphicsSettings {
    pub scale: f32,
}