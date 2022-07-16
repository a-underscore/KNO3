pub mod bullet;

pub use bullet::Bullet;

use gdnative::prelude::*;

pub fn init(init: &InitHandle) {
    init.add_class::<Bullet>();
}
