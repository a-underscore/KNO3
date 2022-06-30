use gdnative::{api::Sprite, prelude::*};

#[derive(NativeClass)]
#[inherit(Sprite)]
pub struct PlayerSprite;

#[methods]
impl PlayerSprite {
    fn new(_owner: &Sprite) -> Self {
        Self
    }
}
