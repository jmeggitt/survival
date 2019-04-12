use std::time::Duration;
use std::time::SystemTime;

use amethyst::assets::Handle;
use amethyst::core::math::Vector2;
use amethyst::renderer::Sprite;
use amethyst::renderer::Texture;
use derivative::Derivative;
use log::warn;
use ncollide2d::shape::{Ball, ShapeHandle};
use ncollide2d::world::CollisionGroups;
use nphysics2d::math::{Inertia, Velocity};
use nphysics2d::object::{BodyStatus, ColliderDesc, RigidBodyDesc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct WorldEntity {
    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    display: EntityGraphics,
    pub pos: Vector2<f64>,
}

impl WorldEntity {
    pub fn new(pos: Vector2<f64>) -> Self {
        //        let collider = ColliderDesc::new();
        //        let physics = RigidBodyDesc::new()
        //            .translation(pos)
        //            .gravity_enabled(false)
        //            .status(BodyStatus::Static);

        Self {
            display: EntityGraphics::None,
            pos: Vector2::new(0.0, 0.0),
        }
    }
}

pub enum EntityGraphics {
    Fixed(Handle<Texture>, Sprite),
    Animate(SpriteAnimation),
    None,
}

// TODO make serializing work for entities
impl Default for EntityGraphics {
    fn default() -> Self {
        unimplemented!()
    }
}

const DEFAULT_FRAME_TIME: Duration = Duration::from_millis(400);

/// Stores sprite animations
/// TODO use sprite animations to draw player
#[allow(dead_code)]
pub struct SpriteAnimation {
    current_loop: usize,
    desired_loop: usize,
    index: usize,
    loops: Vec<FrameLoop>,
    last_transition: SystemTime,
}

impl SpriteAnimation {
    /// Gets the next valid frame to use in this animation
    pub fn next_frame(&mut self) -> (usize, usize) {
        match &self.loops[self.current_loop][self.index].transition {
            FrameTransition::MustMove(loop_index, index) => (*loop_index, *index),
            FrameTransition::CanMove(allowed) => {
                if self.desired_loop != self.current_loop {
                    for (loop_index, index) in allowed {
                        if *loop_index == self.desired_loop {
                            return (*loop_index, *index);
                        }
                    }
                }
                (
                    self.current_loop,
                    (self.index + 1) % self.loops[self.current_loop].len(),
                )
            }
        }
    }

    /// Sets the transition goal of the animation.
    pub fn set_desired_loop(&mut self, desired: usize) {
        self.desired_loop = desired;
    }

    /// Sets the current keyframe. This could be used to transition to an attack or non-periodic
    /// animation which ends in a CanMove which will exit the animation when finished.
    pub fn set_frame(&mut self, loop_index: usize, index: usize) {
        self.current_loop = loop_index;
        self.index = index;
    }
}

/// Animation step
pub struct KeyFrame {
    sprite: Sprite,
    texture: Handle<Texture>,
    time: Duration,
    transition: FrameTransition,
}

impl KeyFrame {
    pub fn new(sprite: Sprite, texture: Handle<Texture>) -> Self {
        Self {
            sprite,
            texture,
            time: DEFAULT_FRAME_TIME,
            transition: FrameTransition::CanMove(Vec::new()),
        }
    }

    pub fn with_duration(&mut self, millis: u64) -> &mut Self {
        self.time = Duration::from_millis(millis);
        self
    }

    pub fn with_transition(&mut self, loop_index: usize, index: usize) -> &mut Self {
        match &mut self.transition {
            FrameTransition::CanMove(moves) => moves.push((loop_index, index)),
            FrameTransition::MustMove(_, _) => {
                warn!("Attempted to add extra transition to must move keyframe!")
            }
        }
        self
    }

    pub fn with_move_transition(&mut self, loop_index: usize, index: usize) -> &mut Self {
        if let FrameTransition::CanMove(x) = &self.transition {
            if x.is_empty() {
                self.transition = FrameTransition::MustMove(loop_index, index);
                return self;
            }
        }
        warn!("Attempted to add extra must move transition to keyframe!");
        self
    }
}

/// Frame loops can always be iterated through and may contain exit points.
pub type FrameLoop = Vec<KeyFrame>;

/// Valid next frames to transition to.
pub enum FrameTransition {
    // Requires the next frame to a specific KeyFrame.
    MustMove(usize, usize),
    // Allowed transitions to other loops. Any transitions to the current loop will be ignored in
    // favor of the next frame in the loop.
    CanMove(Vec<(usize, usize)>),
}
