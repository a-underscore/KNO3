use gdnative::{api::KinematicBody2D, prelude::*};

const VELOCITY_SCALE: f32 = 100.0;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct PlayerBody {
    velocity: Vector2,
}

#[methods]
impl PlayerBody {
    fn new(_owner: &KinematicBody2D) -> Self {
        Self {
            velocity: Vector2::ZERO,
        }
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, _delta: f64) {
        self.velocity_from_input(owner);

        owner.move_and_slide(
            self.velocity * VELOCITY_SCALE,
            Vector2::new(0.0, -1.0),
            false,
            4,
            0.785398,
            true,
        );
    }

    fn velocity_from_input(&mut self, owner: &KinematicBody2D) {
        let input = Input::godot_singleton();

        if input.is_action_pressed("player_right", false) {
            self.velocity.x = 10.0;
        } else if input.is_action_pressed("player_left", false) {
            self.velocity.x = -10.0;
        } else {
            self.velocity.x = 0.0;
        }

        if input.is_action_just_pressed("player_jump", false) && owner.is_on_floor() {
            self.velocity.y = -10.0;
        } else if input.is_action_pressed("player_jump", false) && self.velocity.y < 1.0 {
            self.velocity.y += 0.5;
        } else {
            self.velocity.y = 9.8;
        }
    }
}
