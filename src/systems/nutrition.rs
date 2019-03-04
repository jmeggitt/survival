#![allow(clippy::module_name_repetitions)]
use amethyst::{
    ecs::{Join, Component, FlaggedStorage, DenseVecStorage, Resources, SystemData, ReadExpect, WriteStorage, storage::ComponentEvent, },
    shrev::{EventChannel, ReaderId},
};

use crate::settings::Context;
use crate::systems::time::TimeState;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
enum Vitamin {

}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Food {
    pub vitamins: [f32; 25],
    pub calories: u32,
    pub sugars: u32,
    pub fats: u32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Nutrition {
    pub vitamins: [f32; 25],
    pub stomach_load: f32,
    pub upper_intestine_load: f32,
    pub lower_intestine_load: f32,
    pub colon_load: f32,

    pub bladder_load: f32,
    pub hydration: f32,

    pub caloric_balance: f32,
    pub caloric_track: u32,

    #[serde(skip_serializing, skip_deserializing)]
    pub consume: EventChannel<Food>,
}
impl Component for Nutrition { type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>; }


#[derive(Default)]
pub struct System {
    components: hibitset::BitSet,
    comp_reader_id: Option<ReaderId<ComponentEvent>>,
    elapsed_event_reader_id: Option<ReaderId<u64>>
}
impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        ReadExpect<'s, TimeState>,
        WriteStorage<'s, Nutrition>
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.comp_reader_id = Some(res.fetch_mut::<EventChannel<ComponentEvent>>().register_reader());
        self.elapsed_event_reader_id = Some(res.fetch_mut::<TimeState>().elapsed_event.register_reader());
    }

    fn run(&mut self, (_, time, nutritions): Self::SystemData) {
        for event in nutritions.channel().read(self.comp_reader_id.as_mut().unwrap()) {
            match event {
                ComponentEvent::Inserted(id) => { self.components.add(*id); }
                ComponentEvent::Removed(id) => { self.components.remove(*id); },
                _ => {},
            };
        }
        for time_elapsed in time.elapsed_event.read(self.elapsed_event_reader_id.as_mut().unwrap()) {
            for (data, _) in (&nutritions, &self.components).join() {
                // Handle data

                // Do nutirtion shit based on the time elapsed
            }
        }

    }
}