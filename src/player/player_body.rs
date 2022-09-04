use crate::GRAVITY;
use gdnative::{
    api::{AnimatedSprite, CollisionShape2D},
    prelude::*,
};
use std::time::{Duration, Instant};

const VELOCITY_SCALE: f32 = 100.0;
const MOVE_SPEED: Vector2 = Vector2::new(10.0, 5.0);
const JUMP_DECAY: f32 = 2.5;
const DASH_MODIFIER: f32 = 10.0;
const DASH_TIME: Duration = Duration::from_millis(500);
const JUMP_TIME: Duration = Duration::from_millis(100);

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct PlayerBody {
    velocity: Vector2,
    dash_time: Instant,
    jump_time: Instant,
}

#[methods]
impl PlayerBody {
    fn new(_owner: &KinematicBody2D) -> Self {
        Self {
            velocity: Vector2::ZERO,
            dash_time: Instant::now(),
            jump_time: Instant::now(),
        }
    }

    #[method]
    fn _physics_process(&mut self, #[base] owner: &KinematicBody2D, _delta: f64) {
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

            self.set_animation(owner, &"running".to_string());
        } else if input.is_action_pressed("player_left", false) {
            self.velocity.x = -MOVE_SPEED.x;

            self.set_animation(owner, &"running".to_string());
        } else {
            self.velocity.x = 0.0;

            self.set_animation(owner, &"default".to_string());
        }

        if owner.is_on_ceiling() {
            self.velocity.y *= -1.0;
        } else {
            if owner.is_on_floor() {
                if input.is_action_just_pressed("player_jump", false) {
                    self.velocity.y = -MOVE_SPEED.y;
                    self.jump_time = Instant::now();
                } else {
                    self.velocity.y = 0.0;
                }
            } else if input.is_action_pressed("player_jump", false)
                && Instant::now().duration_since(self.jump_time) <= JUMP_TIME
            {
                self.velocity.y -= JUMP_DECAY;
            }
        }

        self.velocity.y += GRAVITY;

        if input.is_action_just_pressed("player_dash", false)
            && Instant::now().duration_since(self.dash_time) >= DASH_TIME
        {
            let cs_scale = unsafe { owner.get_node_as::<CollisionShape2D>("./PlayerCS") }
                .unwrap()
                .get_transform()
                .scale()
                .y;

            self.velocity.x += DASH_MODIFIER * MOVE_SPEED.x * cs_scale;
            self.dash_time = Instant::now();
        }
    }

    fn set_animation(&self, owner: &KinematicBody2D, animation: &String) {
        unsafe { owner.get_node_as::<AnimatedSprite>("./PlayerCS/PlayerSprite") }
            .unwrap()
            .set_animation(animation);
    }
}
