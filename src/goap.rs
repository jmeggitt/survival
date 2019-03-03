use amethyst::ecs::{Entity, storage::UnprotectedStorage, DenseVecStorage, world::Index, World };

trait HasId {
    fn id(&self, ) -> Index;
}


#[derive(Copy, Clone)]
pub struct Action<'a>
{
    pub id: Index,
    pub cost: &'a Fn(&Entity, &World) -> u32,
    pub pre_condition: &'a Fn(&Entity, &World) -> bool,
    pub effect: &'a Fn(&Entity, &mut World),
}
impl<'a> HasId for Action<'a> {
    fn id(&self, ) -> Index { self.id }
}
impl<'a> PartialEq for Action<'a> {
    fn eq(&self, other: &Action<'a>) -> bool {
        self.id() == other.id()
    }
}


pub struct Planner<'a>
{
    cur_id: Index,
    actions: DenseVecStorage<Action<'a>>,
}

impl<'a> Planner<'a>
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_action(&mut self, cost: &'a Fn(&Entity, &World) -> u32, pre_condition: &'a Fn(&Entity, &World) -> bool, effect: &'a Fn(&Entity, &mut World),) -> Index {
        let id = self.cur_id;

        unsafe {
            self.actions.insert(id, Action{ id, cost, pre_condition, effect,  });
        }

        self.cur_id += 1;
        id
    }

    pub fn get(&self, id: Index) -> Option<&Action<'a>> {
        if id > self.cur_id {
            return None;
        }
        unsafe {
            Some(self.actions.get(id))
        }
    }
    pub fn get_mut(&mut self, id: Index) -> Option<&mut Action<'a>> {
        if id > self.cur_id {
            return None;
        }
        unsafe {
            Some(self.actions.get_mut(id))
        }
    }

    pub fn can_occur(&self, id: Index, entity: Entity, world: &World) -> bool {
        match self.get(id) {
            Some(a) => {
                (a.pre_condition)(&entity, world)
            },
            None => false,
        }
    }

    pub fn cost(&self, id: Index, entity: Entity, world: &World) -> u32 {
        match self.get(id) {
            Some(a) => {
                (a.cost)(&entity, world)
            },
            None => u32::max_value(),
        }
    }

    pub fn apply(&self, id: Index, entity: Entity, world: &mut World) {
        if let Some(a) = self.get(id) {
            (a.effect)(&entity, world)
        }
    }
}

impl<'a> Default for Planner<'a>
{
    fn default() -> Self {
        Self {
            actions: DenseVecStorage::default(),
            cur_id: 0,
        }
    }
}