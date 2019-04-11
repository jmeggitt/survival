use amethyst::ecs::{self, prelude::*, shred::ResourceId};

pub trait EventSystem<'a> {
    type SystemData: ecs::SystemData<'a>;
    type Event: Sized;

    fn run(data: &Self::SystemData, event: &mut Self::Event) -> bool;

    fn invalidate(_data: &Self::SystemData, _event: &mut Self::Event) {}
}

impl<'a, E, A, B> EventSystem<'a> for (A, B)
    where
        A: EventSystem<'a, Event=E>,
        B: EventSystem<'a, Event=E>,
{
    type SystemData = (A::SystemData, B::SystemData);
    type Event = E;

    fn run(data: &Self::SystemData, event: &mut Self::Event) -> bool {
        if A::run(&data.0, event) {
            if B::run(&data.1, event) {
                return true;
            } else {
                A::invalidate(&data.0, event);
            }
        }

        false
    }
}

impl<'a, E, A, B, C> EventSystem<'a> for (A, B, C)
    where
        A: EventSystem<'a, Event=E>,
        B: EventSystem<'a, Event=E>,
        C: EventSystem<'a, Event=E>,
{
    type SystemData = (A::SystemData, B::SystemData, C::SystemData);
    type Event = E;

    fn run(data: &Self::SystemData, event: &mut Self::Event) -> bool {
        if A::run(&data.0, event) {
            if B::run(&data.1, event) {
                if C::run(&data.2, event) {
                    return true;
                } else {
                    B::invalidate(&data.1, event);
                    A::invalidate(&data.0, event);
                }
            } else {
                A::invalidate(&data.0, event);
            }
        }

        false
    }
}

impl<'a, E, A, B, C, D> EventSystem<'a> for (A, B, C, D)
    where
        A: EventSystem<'a, Event=E>,
        B: EventSystem<'a, Event=E>,
        C: EventSystem<'a, Event=E>,
        D: EventSystem<'a, Event=E>,
{
    type SystemData = (A::SystemData, B::SystemData, C::SystemData, D::SystemData);
    type Event = E;

    fn run(data: &Self::SystemData, event: &mut Self::Event) -> bool {
        if A::run(&data.0, event) {
            if B::run(&data.1, event) {
                if C::run(&data.2, event) {
                    if D::run(&data.3, event) {
                        return true;
                    } else {
                        C::invalidate(&data.2, event);
                        B::invalidate(&data.1, event);
                        A::invalidate(&data.0, event);
                    }
                } else {
                    B::invalidate(&data.1, event);
                    A::invalidate(&data.0, event);
                }
            } else {
                A::invalidate(&data.0, event);
            }
        }

        false
    }
}

pub struct ReifiedEventSystem<'a, T>
    where
        T: EventSystem<'a>,
{
    data: T::SystemData,
}

impl<'a, T> ReifiedEventSystem<'a, T>
    where
        T: EventSystem<'a>,
{
    pub fn run(&self, event: &mut T::Event) -> bool {
        T::run(&self.data, event)
    }
}

impl<'a, T> SystemData<'a> for ReifiedEventSystem<'a, T>
    where
        T: EventSystem<'a>,
{
    fn setup(res: &mut Resources) {
        T::SystemData::setup(&mut *res);
    }

    fn fetch(res: &'a Resources) -> Self {
        ReifiedEventSystem {
            data: T::SystemData::fetch(res),
        }
    }

    fn reads() -> Vec<ResourceId> {
        T::SystemData::reads()
    }

    fn writes() -> Vec<ResourceId> {
        T::SystemData::writes()
    }
}
