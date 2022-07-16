use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Rooms;

#[methods]
impl Rooms {
    fn new(_owner: &Node2D) -> Self {
        Self
    }
}
