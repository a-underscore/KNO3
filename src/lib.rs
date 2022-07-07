pub mod player;

use gdnative::prelude::*;
use player::{player_arms::PlayerArms, player_body::PlayerBody, player_cs::PlayerCS};

fn init(handle: InitHandle) {
    handle.add_class::<PlayerBody>();
    handle.add_class::<PlayerCS>();
    handle.add_class::<PlayerArms>();
}

godot_init!(init);
