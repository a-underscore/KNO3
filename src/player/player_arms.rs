use gdnative::prelude::*;
use std::f64::consts::PI;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct PlayerArms;

#[methods]
impl PlayerArms {
    fn new(_owner: &Node2D) -> Self {
        Self
    }

    #[export]
    fn _process(&mut self, owner: &Node2D, _delta: f64) {
        let mouse_pos = owner.get_global_mouse_position();

        owner.look_at(mouse_pos);
        owner.rotate(PI);
    }
}
