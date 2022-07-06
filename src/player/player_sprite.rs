use gdnative::{api::Sprite, prelude::*};

#[derive(NativeClass)]
#[inherit(Sprite)]
pub struct PlayerSprite;

#[methods]
impl PlayerSprite {
    fn new(_owner: &Sprite) -> Self {
        Self
    }

    #[export]
    fn _physics_process(&mut self, owner: &Sprite, _delta: f64) {
        let viewport = unsafe { owner.get_viewport().unwrap().assume_safe() };
        let flip_point = viewport.size().x / 2.0;
        let mouse_position = viewport.get_mouse_position();
        let scale = owner.scale();

        if mouse_position.x < flip_point {
            owner.set_scale(Vector2::new(-1.0, scale.y));
        } else if mouse_position.x > flip_point {
            owner.set_scale(Vector2::new(1.0, scale.y));
        }
    }
}
