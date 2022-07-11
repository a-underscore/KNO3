use crate::GRAVITY;
use gdnative::prelude::*;
use std::time::{Duration, Instant};

const VELOCITY_SCALE: f32 = 100.0;
const MOVE_SPEED: Vector2 = Vector2::new(10.0, 20.0);
const JUMP_DECAY: f32 = 1.5;
const DASH_MODIFIER: f32 = 25.0;
const DASH_TIME: Duration = Duration::from_millis(1000);

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct PlayerBody {
    velocity: Vector2,
    dash_time: Instant,
}
#[methods]
impl PlayerBody {
    fn new(_owner: &KinematicBody2D) -> Self {
        Self {
            velocity: Vector2::ZERO,
            dash_time: Instant::now(),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &KinematicBody2D) {
        self.dash_time = Instant::now();
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, _delta: f64) {
        self.velocity_from_input(owner);

        owner.move_and_slide(
            self.velocity * VELOCITY_SCALE,
            Vector2::UP,
            false,
            4,
            0.785398,
            true,
        );
    }

    fn velocity_from_input(&mut self, owner: &KinematicBody2D) {
        let input = Input::godot_singleton();

        if input.is_action_pressed("player_right", false) {
            self.velocity.x = MOVE_SPEED.x;

            self.attempt_dash(&input, 1.0);
        } else if input.is_action_pressed("player_left", false) {
            self.velocity.x = -MOVE_SPEED.x;

            self.attempt_dash(&input, -1.0);
        } else {
            self.velocity.x = 0.0;
        }

        if input.is_action_just_pressed("player_jump", false) && owner.is_on_floor() {
            self.velocity.y = -MOVE_SPEED.y;
        } else if input.is_action_pressed("player_jump", false) && self.velocity.y < 1.0 {
            self.velocity.y += JUMP_DECAY;
        } else {
            self.velocity.y = GRAVITY;
        }
    }

    fn attempt_dash(&mut self, input: &Input, modifier: f32) {
        if input.is_action_just_pressed("player_dash", false)
            && Instant::now().duration_since(self.dash_time) >= DASH_TIME
        {
            self.velocity.x += DASH_MODIFIER * MOVE_SPEED.x * modifier;
            self.dash_time = Instant::now();
        }
    }
}
