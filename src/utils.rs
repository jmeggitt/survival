use amethyst::shrev;
use amethyst::ecs::{
    Entity, Resources, WriteStorage, Component, SystemData, Entities, Join,
    storage::{ComponentEvent, FlaggedStorage, UnprotectedStorage},
    storage,
};
use hibitset::BitSet;

pub trait HasChannel<E> {
    /// Event channel tracking modified/inserted/removed components.
    fn channel(&self) -> &shrev::EventChannel<E>;
    /// Mutable event channel tracking modified/inserted/removed components.
    fn channel_mut(&mut self) -> &mut shrev::EventChannel<E>;
}

#[derive(Default)]
pub struct ComponentEventReader<C, T, S>
    where
        T: 'static,
{
    component_reader: Option<shrev::ReaderId<ComponentEvent>>,
    action_readers: std::collections::HashMap<Entity, shrev::ReaderId<T>>,
    phantom_1: std::marker::PhantomData<C>,
    phantom_2: std::marker::PhantomData<S>,
    components: BitSet,
}

impl<C, T, S> ComponentEventReader<C, T, S>
    where
        T: amethyst::shrev::Event + 'static,
        C: Component<Storage=S> + HasChannel<T> + Sized,
        S: UnprotectedStorage<C> + storage::Tracked + Sized + Send + Sync + 'static,
{
    pub fn setup(&mut self, res: &mut Resources, ) {
        let reader_id = WriteStorage::<C>::fetch(&res).channel_mut().register_reader();
        self.component_reader = Some(reader_id);
    }

    pub fn subscribe(&mut self, entity: Entity, storage: &mut WriteStorage<C>, ) {
        self.action_readers.insert(entity, storage.get_mut(entity).unwrap().channel_mut().register_reader());
    }

    pub fn unsubscribe(&mut self, entity: Entity) {
        self.action_readers.remove(&entity);
    }

    pub fn maintain(&mut self, entities: &Entities, storage: &mut WriteStorage<C>) {
        let mut comp_remove = BitSet::new();
        let mut comp_new = BitSet::new();

        for event in storage.channel().read(self.component_reader.as_mut().unwrap()) {
            match event {
                ComponentEvent::Inserted(id) => {
                    self.components.add(*id);
                    comp_new.add(*id);
                }
                ComponentEvent::Removed(id) => {
                    comp_remove.add(*id);
                },
                _ => {},
            }
        }

        for (entity, _) in (entities, comp_remove).join() {
            self.unsubscribe(entity);
        }

        for (entity, _) in (entities, comp_new).join() {
            self.subscribe(entity, storage);
        }
    }

    pub fn read_storage<'a>(&mut self, entity: Entity, storage: &'a mut WriteStorage<'a, C>) -> shrev::EventIterator<'a, T> {
        storage.get(entity).unwrap().channel().read(self.action_readers.get_mut(&entity).unwrap())
    }

    pub fn read<'a>(&mut self, entity: Entity, component: &'a mut C) -> shrev::EventIterator<'a, T> {
        component.channel().read(self.action_readers.get_mut(&entity).unwrap())
    }
}