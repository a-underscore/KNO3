mod bullet;
mod player;

use gdnative::prelude::*;

pub const GRAVITY: f32 = 9.8;

fn init(handle: InitHandle) {
    bullet::init(&handle);
    player::init(&handle);
}

godot_init!(init);
