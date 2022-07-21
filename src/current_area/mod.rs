pub mod current_area;
pub mod room_data;

pub use current_area::CurrentArea;

use gdnative::prelude::*;

pub fn init(init: &InitHandle) {
    init.add_class::<CurrentArea>();
}
