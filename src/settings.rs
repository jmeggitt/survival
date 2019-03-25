use serde::{Deserialize, Serialize};

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

pub type Context = Option<amethyst::renderer::SpriteSheetHandle>;
