#![allow(clippy::module_name_repetitions)]
use amethyst::{
    ecs::{Entity, Component, Read, DenseVecStorage, Resources, SystemData, ReadExpect, WriteStorage,  },
    shrev::{EventChannel, ReaderId},
};
use specs_derive::Component;
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

#[derive(Component, serde::Serialize, serde::Deserialize)]
#[storage(DenseVecStorage)]
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

    pub config: NutritionConfig,
}
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

            config: NutritionConfig::default(),
        }
    }
}

#[derive(Default)]
pub struct System {
    consume_reader_id: Option<ReaderId<(Entity, Food)>>,
    elapsed_event_reader_id: Option<ReaderId<u64>>
}
impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        Read<'s, TimeState>,
        Read<'s, EventChannel<(Entity, Food)>>,
        WriteStorage<'s, Nutrition>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.consume_reader_id = Some(res.fetch_mut::<EventChannel<(Entity, Food)>>().register_reader());
        self.elapsed_event_reader_id = Some(res.fetch_mut::<TimeState>().elapsed_event.register_reader());
    }

    fn run(&mut self, (_, time, consume_events, mut nutritions): Self::SystemData) {
        // Check for elapsed time
        for _time_elapsed in time.elapsed_event.read(self.elapsed_event_reader_id.as_mut().unwrap()) {
            for (entity, _food) in consume_events.read(self.consume_reader_id.as_mut().unwrap()) {
                if let Some(_nutrition) = nutritions.get_mut(*entity) {

                }
            }
        }
    }
}