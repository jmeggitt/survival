use image;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpritePosition {
    /// Horizontal position of the sprite in the sprite sheet
    pub x: u32,
    /// Vertical position of the sprite in the sprite sheet
    pub y: u32,
    /// Width of the sprite
    pub width: u32,
    /// Height of the sprite
    pub height: u32,
    /// Number of pixels to shift the sprite to the left and down relative to the entity holding it
    pub offsets: Option<[f32; 2]>,
}

/// Structure acting as scaffolding for serde when loading a spritesheet file.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SerializedSpriteSheet {
    /// Width of the sprite sheet
    pub spritesheet_width: u32,
    /// Height of the sprite sheet
    pub spritesheet_height: u32,
    /// Description of the sprites
    pub sprites: Vec<SpritePosition>,
}

fn get_tile_size(filepart: &str) -> usize {
    // Last 2 digits of a filepart should be the size right?!
    let size_str = &filepart[filepart.len()-2..filepart.len()];
    size_str.parse::<usize>().unwrap()
}

fn main() {
    use std::io::Write;
    use std::fs::{self, DirEntry};
    use std::path::Path;
    use image::GenericImageView;

    // one possible implementation of walking a directory only visiting files
    for entry in fs::read_dir(&Path::new("resources/spritesheets/")).unwrap() {
        match entry {
            Ok(file) => {
                let input_path = file.path();
                if input_path.is_file() && match input_path.extension() { Some(e) => e == "png", None => false } {
                    match image::open(input_path.as_path()) {
                        Ok(img) => {
                            println!("rerun-if-changed={}", file.path().to_string_lossy());

                            // Extract the dimensions from the filename
                            let stride = get_tile_size(input_path.as_path().file_stem().unwrap().to_str().unwrap());

                            let mut sprites = Vec::new();

                            for x in (0..img.dimensions().0).step_by(stride) {
                                for y in (0..img.dimensions().1).step_by(stride){
                                    sprites.push(SpritePosition {
                                        offsets: None,
                                        x,
                                        y,
                                        width: stride as u32,
                                        height: stride as u32,
                                    })
                                }
                            }

                            let mut sheet = SerializedSpriteSheet {
                                spritesheet_width: img.dimensions().0,
                                spritesheet_height: img.dimensions().1,
                                sprites,
                            };

                            // Write it
                            let pretty = ron::ser::PrettyConfig {
                                depth_limit: 99,
                                separate_tuple_members: true,
                                enumerate_arrays: true,
                                ..ron::ser::PrettyConfig::default()
                            };
                            let s = ron::ser::to_string_pretty(&sheet, pretty).expect("Serialization failed");

                            let mut output_path = input_path.as_path().with_extension("ron");
                            let mut file = std::fs::File::create(output_path.as_path()).unwrap();

                            file.write_all(s.as_bytes()).unwrap();

                        },
                        Err(_) => {},
                    }
                }
            },
            Err(_) => {},
        }
    }
}