use amethyst::{
    ecs::{World, Dispatcher, DispatcherBuilder, System},
    DataInit,
    Result,
    core::{
        bundle::SystemBundle,
        ArcThreadPool
    }
};

pub struct SurvivalData<'a, 'b> {
    level_dispatcher: Dispatcher<'a, 'b>,
    overworld_dispatcher: Dispatcher<'a, 'b>,
    core_dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> SurvivalData<'a, 'b> {
    /// Update game data
    pub fn update(&mut self, world: &World) {
        self.level_dispatcher.dispatch(&world.res);
        self.overworld_dispatcher.dispatch(&world.res);
        self.core_dispatcher.dispatch(&world.res);
    }
}

pub struct SurvivalDataBuilder<'a, 'b> {
    pub level_dispatcher: DispatcherBuilder<'a, 'b>,
    pub overworld_dispatcher: DispatcherBuilder<'a, 'b>,
    pub core_dispatcher: DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> Default for SurvivalDataBuilder<'a, 'b> {
    fn default() -> Self {
        SurvivalDataBuilder::new()
    }
}

impl<'a, 'b> SurvivalDataBuilder<'a, 'b> {
    pub fn new() -> Self {
        Self {
            level_dispatcher: DispatcherBuilder::new(),
            overworld_dispatcher: DispatcherBuilder::new(),
            core_dispatcher: DispatcherBuilder::new(),
        }
    }

    pub fn with_base_bundle<B>(mut self, bundle: B) -> Result<Self>
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

        let mut core_dispatcher = self.core_dispatcher.with_pool(pool.clone()).build();
        let mut level_dispatcher = self.level_dispatcher.with_pool(pool.clone()).build();
        let mut overworld_dispatcher = self.overworld_dispatcher.with_pool(pool.clone()).build();

        core_dispatcher.setup(&mut world.res);
        level_dispatcher.setup(&mut world.res);
        overworld_dispatcher.setup(&mut world.res);

        SurvivalData { core_dispatcher, level_dispatcher, overworld_dispatcher }
    }
}