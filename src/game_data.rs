use amethyst::{
    core::{bundle::SystemBundle, ArcThreadPool},
    ecs::{Dispatcher, DispatcherBuilder, System, World},
    DataInit, Result,
};

#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    strum_macros::EnumString,
    strum_macros::Display,
)]
pub enum SurvivalState {
    Paused,
    Running,
    // Unused
    Level,
}

impl Default for SurvivalState {
    fn default() -> Self {
        SurvivalState::Paused
    }
}

pub struct SurvivalData<'a, 'b> {
    level_dispatcher: Dispatcher<'a, 'b>,
    overworld_dispatcher: Dispatcher<'a, 'b>,
    core_dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> SurvivalData<'a, 'b> {
    /// Update game data
    pub fn update(&mut self, world: &World, state: SurvivalState) -> SurvivalState {
        *world.res.fetch_mut::<SurvivalState>() = state;

        self.level_dispatcher.dispatch(&world.res);
        //self.overworld_dispatcher.dispatch(&world.res);
        self.core_dispatcher.dispatch(&world.res);

        world.res.fetch::<SurvivalState>().clone()
    }
}

pub struct SurvivalDataBuilder<'a, 'b> {
    pub level_dispatcher: DispatcherBuilder<'a, 'b>,
    pub overworld_dispatcher: DispatcherBuilder<'a, 'b>,
    pub core_dispatcher: DispatcherBuilder<'a, 'b>,
    pub context: crate::settings::Context,
    pub display_config: amethyst::renderer::DisplayConfig,
    pub game_config: crate::settings::Config,
}

impl<'a, 'b> SurvivalDataBuilder<'a, 'b> {
    pub fn new(
        context: crate::settings::Context,
        display_config: amethyst::renderer::DisplayConfig,
        game_config: crate::settings::Config,
    ) -> Self {
        Self {
            context,
            display_config,
            game_config,
            level_dispatcher: DispatcherBuilder::new(),
            overworld_dispatcher: DispatcherBuilder::new(),
            core_dispatcher: DispatcherBuilder::new(),
        }
    }

    pub fn with_core_bundle<B>(mut self, bundle: B) -> Result<Self>
    where
        B: SystemBundle<'a, 'b>,
    {
        bundle.build(&mut self.core_dispatcher)?;

        Ok(self)
    }

    pub fn with_core<S>(mut self, system: S, name: &str, dependencies: &[&str]) -> Self
    where
        for<'c> S: System<'c> + Send + 'a,
    {
        self.core_dispatcher.add(system, name, dependencies);
        self
    }

    pub fn with_level<S>(mut self, system: S, name: &str, dependencies: &[&str]) -> Self
    where
        for<'c> S: System<'c> + Send + 'a,
    {
        self.level_dispatcher.add(system, name, dependencies);
        self
    }

    pub fn with_overworld<S>(mut self, system: S, name: &str, dependencies: &[&str]) -> Self
    where
        for<'c> S: System<'c> + Send + 'a,
    {
        self.overworld_dispatcher.add(system, name, dependencies);
        self
    }
}

impl<'a, 'b> DataInit<SurvivalData<'a, 'b>> for SurvivalDataBuilder<'a, 'b> {
    fn build(self, world: &mut World) -> SurvivalData<'a, 'b> {
        // Get a handle to the `ThreadPool`.
        let pool = world.read_resource::<ArcThreadPool>().clone();

        // Add global resources
        world.add_resource(self.context);
        world.add_resource(self.game_config);
        world.add_resource(self.display_config);

        // create dispatchers
        let mut core_dispatcher = self.core_dispatcher.with_pool(pool.clone()).build();
        let mut level_dispatcher = self.level_dispatcher.with_pool(pool.clone()).build();
        let mut overworld_dispatcher = self.overworld_dispatcher.with_pool(pool.clone()).build();

        core_dispatcher.setup(&mut world.res);
        level_dispatcher.setup(&mut world.res);
        overworld_dispatcher.setup(&mut world.res);

        // Add the context state to the world

        SurvivalData {
            core_dispatcher,
            level_dispatcher,
            overworld_dispatcher,
        }
    }
}
