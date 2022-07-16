pub mod rooms;

pub use rooms::Rooms;

use gdnative::prelude::*;

pub fn init(init: &InitHandle) {
    init.add_class::<Rooms>();
}
