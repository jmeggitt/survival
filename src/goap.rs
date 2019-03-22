use amethyst::ecs::{Entity, storage::UnprotectedStorage, DenseVecStorage, world::Index, World };

#[derive(Clone)]
pub struct Action {
    name: String,
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}


pub struct Planner {
    cur_action: Index,
    actions: DenseVecStorage<Action>,
}

impl Planner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_action(&mut self, )-> Index {
        let action = self.cur_action;

        self.cur_action += 1;
        action
    }

    pub fn get(&self, action: Index) -> Option<&Action> {
        if action > self.cur_action {
            return None;
        }
        unsafe {
            Some(self.actions.get(action))
        }
    }
    pub fn get_mut(&mut self, action: Index) -> Option<&mut Action> {
        if action > self.cur_action {
            return None;
        }
        unsafe {
            Some(self.actions.get_mut(action))
        }
    }

    pub fn can_occur(&self, _action: Index, _entity: Entity, _world: &World) -> bool {
        false
    }

    pub fn cost(&self, _action: Index, _entity: Entity, _world: &World) -> u32 {
        0
    }

    pub fn apply(&self, _action: Index, _entity: Entity, _world: &mut World) {

    }
}

impl Default for Planner
{
    fn default() -> Self {
        Self {
            actions: DenseVecStorage::default(),
            cur_action: 0,
        }
    }
}