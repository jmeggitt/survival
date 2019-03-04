use bitflags::*;

use serde::{Serialize, Deserialize};
use amethyst::{
    ecs::VecStorage,
    assets::{Handle, Asset}
};
use std::sync::Arc;
use std::collections::HashMap;

mod loader;
#[allow(unused_imports)]
use loader::AssetLoader as AssetLoader;

pub mod body;

bitflags_serial! {
    pub struct ItemFlag: u64 {
        const None = 1 << 0;
        const Container = 1 << 2;
        const Tool = 1 << 3;
    }
}

bitflags_serial! {
    pub struct ContainerCanHold: u8 {
        const Nothing = 1 << 0;
        const Liquid = 1 << 1;
        const Solid  = 1 << 2;
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ItemProperty {
    Container {
        can_hold: ContainerCanHold,
    },
    Chopping(f32),
    Cutting(f32),
    Hitting(f32),
    Hammering(f32),
    Cooking(f32),
    Boiling(f32),
    None,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct ItemDetails {
    // general information
    pub size: (f32, f32, f32),
    pub weight: f32,
    pub flags: ItemFlag,

    // UI information
    pub name: String,
    pub short_description: String,
    pub long_description: String,
    pub sprite_sheet_number: u32,
    pub sprite_number: u32,

    pub properties: Vec<ItemProperty>,
    pub interactions: crate::components::InteractionType,
}
impl PartialEq for ItemDetails { fn eq(&self, other: &Self) -> bool { self.name == other.name } }

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct ItemStorage {
    tag: u32,
    items: HashMap<String, Arc<ItemDetails>>,
}
impl Asset for ItemStorage {
    const NAME: &'static str = "survival::Items";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

#[test]
pub fn write_test_collection() {
    use std::fs::{OpenOptions};
    use std::path::Path;
    use std::io::Write;

    let mut collection = ItemStorage::default();

    collection.items.insert("test_collection_item_1".to_string(), Arc::new(ItemDetails {
        name: "Test Collection Item 1".to_owned(),
        short_description: "Test Collection Item 1".to_owned(),
        long_description: "Test Collection Item 1".to_owned(),
        ..Default::default()
    }));
    collection.items.insert("test_collection_item_2".to_string(), Arc::new(ItemDetails {
        name: "Test Collection Item 2".to_owned(),
        short_description: "Test Collection Item 2".to_owned(),
        long_description: "Test Collection Item 2".to_owned(),
        flags: ItemFlag::Container,
        properties: vec![ItemProperty::Container { can_hold: ContainerCanHold::Solid }],
        ..Default::default()
    }));

    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(Path::new("resources/data/test.items.ron")).unwrap();
    let serialized = ron::ser::to_string_pretty(&collection, ron::ser::PrettyConfig {
        depth_limit: 4,
        separate_tuple_members: false,
        enumerate_arrays: false,
        ..ron::ser::PrettyConfig::default()
    }).unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
}

