mod bullet;
mod player;

use gdnative::prelude::*;

fn init(handle: InitHandle) {
    bullet::init(&handle);
    player::init(&handle);
}

godot_init!(init);
