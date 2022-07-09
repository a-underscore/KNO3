use gdnative::prelude::*;

const VELOCITY_SCALE: f32 = 2500.0;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Bullet {
    lifetime_count: usize,
}

#[methods]
impl Bullet {
    fn new(_owner: &KinematicBody2D) -> Self {
        Self { lifetime_count: 0 }
    }

    #[export]
    fn _process(&mut self, owner: &KinematicBody2D, delta: f64) {
        if self.lifetime_count >= 500 {
            unsafe { owner.assume_unique() }.queue_free();
        } else {
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
            } else {
                self.lifetime_count += 1;
            }
        }
    }
}
