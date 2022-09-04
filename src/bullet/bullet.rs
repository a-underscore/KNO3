use gdnative::prelude::*;
use std::time::{Duration, Instant};

const VELOCITY_SCALE: f32 = 2500.0;
const LIFE_TIME: Duration = Duration::from_millis(500);

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Bullet {
    instance_time: Instant,
}

#[methods]
impl Bullet {
    fn new(_owner: &KinematicBody2D) -> Self {
        Self {
            instance_time: Instant::now(),
        }
    }

    #[method]
    fn _ready(&mut self, #[base] _owner: &KinematicBody2D) {
        self.instance_time = Instant::now();
    }

    #[method]
    fn _process(&self, #[base] owner: &KinematicBody2D, _delta: f64) {
        if Instant::now().duration_since(self.instance_time) >= LIFE_TIME {
            unsafe { owner.assume_unique() }.queue_free();
        }
    }

    #[method]
    fn _physics_process(&mut self, #[base] owner: &KinematicBody2D, delta: f64) {
        let collide = owner.move_and_collide(
            Vector2::new(-1.0, 0.0).rotated(owner.rotation() as f32)
                * VELOCITY_SCALE
                * delta as f32,
            true,
            true,
            false,
        );

        if let Some(_) = collide {
            unsafe { owner.assume_unique() }.queue_free()
        }
    }
}
