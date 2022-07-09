pub mod player_arms;
pub mod player_body;
pub mod player_cs;
pub mod player_gun;

use gdnative::prelude::*;
use player_arms::PlayerArms;
use player_body::PlayerBody;
use player_cs::PlayerCS;
use player_gun::PlayerGun;

pub fn init(handle: &InitHandle) {
    handle.add_class::<PlayerArms>();
    handle.add_class::<PlayerBody>();
    handle.add_class::<PlayerCS>();
    handle.add_class::<PlayerGun>();
}
