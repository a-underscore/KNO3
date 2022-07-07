use gdnative::{api::CollisionShape2D, prelude::*};

#[derive(NativeClass)]
#[inherit(CollisionShape2D)]
pub struct PlayerCS;

#[methods]
impl PlayerCS {
    fn new(_owner: &CollisionShape2D) -> Self {
        Self
    }

    #[export]
    fn _process(&mut self, owner: &CollisionShape2D, _delta: f64) {
        let viewport = unsafe { owner.get_viewport().unwrap().assume_safe() };
        let flip_point = viewport.size().x / 2.0;
        let mouse_position = viewport.get_mouse_position();
        let scale = owner.scale();

        if (mouse_position.x < flip_point && scale.x > 0.0)
            || (mouse_position.x > flip_point && scale.x < 0.0)
        {
            owner.set_scale(Vector2::new(scale.x * -1.0, scale.y));
        }
    }
}
