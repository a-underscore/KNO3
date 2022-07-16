pub mod rooms;

use gdnative::prelude::*;

pub fn init(init: &InitHandle) {
    rooms::init(init);
}
