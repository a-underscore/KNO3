pub mod player;

use gdnative::prelude::*;
use player::{player_body::PlayerBody, player_sprite::PlayerSprite};

fn init(handle: InitHandle) {
    handle.add_class::<PlayerSprite>();
    handle.add_class::<PlayerBody>();
}

godot_init!(init);
