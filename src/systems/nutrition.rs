#![allow(clippy::module_name_repetitions)]
use amethyst::{
    ecs::{world, Join, Component, FlaggedStorage, DenseVecStorage, Resources, SystemData, ReadExpect, WriteStorage, storage::ComponentEvent, },
    shrev::{EventChannel, ReaderId},
};
use specs_derive::Component;
use std::collections::HashMap;
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

#[derive(Component, Default, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NutritionConfig {
    pub vitamin_speed: [f32; 25],
    pub stomach_speed: f32,
    pub upper_intestine_speed: f32,
    pub lower_intestine_speed: f32,
    pub colon_speed: f32,

    pub hydration_speed: f32,
    pub calorie_speed: f32,
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

    pub config: NutritionConfig,
}
impl Component for Nutrition { type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>; }
impl Default for Nutrition {
    fn default() -> Self {
        Self {
            vitamins: [1.; 25],
            stomach_load: 0.3,
            upper_intestine_load: 0.3,
            lower_intestine_load: 0.3,
            colon_load: 1.,

            bladder_load: 0.3,
            hydration: 1.,

            caloric_balance: 1.,
            caloric_track: 2000,

            consume: EventChannel::default(),

            config: NutritionConfig::default(),
        }
    }
}

#[derive(Default)]
pub struct System {
    components: hibitset::BitSet,
    new_components: hibitset::BitSet,

    comp_reader_id: Option<ReaderId<ComponentEvent>>,
    consume_reader_ids: HashMap<world::Index, ReaderId<Food>>,
    elapsed_event_reader_id: Option<ReaderId<u64>>
}
impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        ReadExpect<'s, TimeState>,
        WriteStorage<'s, Nutrition>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.comp_reader_id = Some(res.fetch_mut::<EventChannel<ComponentEvent>>().register_reader());
        self.elapsed_event_reader_id = Some(res.fetch_mut::<TimeState>().elapsed_event.register_reader());
    }

    fn run(&mut self, (_, time, mut nutritions): Self::SystemData) {
        self.new_components.clear();

        for event in nutritions.channel().read(self.comp_reader_id.as_mut().unwrap()) {
            match event {
                ComponentEvent::Inserted(id) => {
                    self.components.add(*id);
                    self.new_components.add(*id);
                }
                ComponentEvent::Removed(id) => {
                    self.components.remove(*id);
                    self.consume_reader_ids.remove(id); // Remove the reader
                },
                _ => {},
            };
        }
        // Subscribe to new components
        for (new_component, id) in (&mut nutritions, &self.new_components).join() {
            self.consume_reader_ids.insert(id, new_component.consume.register_reader());
        }


        // Check for elapsed time
        for _time_elapsed in time.elapsed_event.read(self.elapsed_event_reader_id.as_mut().unwrap()) {
            for (data, id) in (&mut nutritions, &self.components).join() {

                // Check for food consumption events, we only do these when time elapsed?
                for _consumed in data.consume.read(self.consume_reader_ids.get_mut(&id).unwrap()) {

                }

                // Do nutirtion shit based on the time elapsed
            }
        }



    }
}