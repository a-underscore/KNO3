pub mod player;

use gdnative::prelude::*;
use player::{player_body::PlayerBody, player_cs::PlayerCS};

fn init(handle: InitHandle) {
    handle.add_class::<PlayerBody>();
    handle.add_class::<PlayerCS>();
}

godot_init!(init);
