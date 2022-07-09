use crate::bullet::Bullet;
use gdnative::prelude::*;
use rand::prelude::*;
use std::{
    f32::consts::PI,
    time::{Duration, Instant},
};

const FIRE_TIME: Duration = Duration::from_millis(50);

#[derive(NativeClass)]
#[inherit(Sprite)]
pub struct PlayerGun {
    bullet: Ref<PackedScene>,
    fired_time: Instant,
}

#[methods]
impl PlayerGun {
    fn new(_owner: &Sprite) -> Self {
        Self {
            bullet: PackedScene::new().into_shared(),
            fired_time: Instant::now(),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Sprite) {
        let loader = ResourceLoader::godot_singleton();

        self.bullet = loader
            .load("res://Bullet/Bullet.tscn", "", false)
            .unwrap()
            .cast()
            .unwrap();
        self.fired_time = Instant::now();
    }

    #[export]
    fn _process(&mut self, owner: &Sprite, _delta: f64) {
        let input = Input::godot_singleton();

        if Instant::now().duration_since(self.fired_time) >= FIRE_TIME {
            if input.is_action_pressed("player_shoot", false) {
                self.fired_time = Instant::now();

                let bullet = unsafe { self.bullet.assume_safe() }
                    .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
                    .unwrap();

                unsafe { bullet.assume_safe() }
                    .cast::<KinematicBody2D>()
                    .unwrap()
                    .cast_instance::<Bullet>()
                    .unwrap()
                    .map(|_, bullet_owner| {
                        bullet_owner.set_global_rotation(
                            owner.global_rotation()
                                + rand::thread_rng().gen_range(-PI / 100.0..PI / 100.0) as f64,
                        );
                        bullet_owner.set_global_position(owner.global_position());
                    })
                    .unwrap();

                unsafe {
                    owner
                        .get_tree()
                        .unwrap()
                        .assume_safe()
                        .cast::<SceneTree>()
                        .unwrap()
                        .root()
                        .unwrap()
                        .assume_safe()
                }
                .add_child(bullet, false);
            }
        }
    }
}
