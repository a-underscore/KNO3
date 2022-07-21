use super::room_data::RoomData;
use gdnative::{api::TileMap, prelude::*};
use itertools::Itertools;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use std::{collections::HashMap, sync::Arc};

const AREA_1_PATH: &str = "Rooms/Area1/area_1/";
const TILE_SIZE: f32 = 27.0;
const GRID_SIZE: f32 = 26.0;
const MAP_SIZE: usize = 27;

lazy_static! {
    static ref AREA_1: Arc<HashMap<String, RoomData>> =
        Arc::new(RoomData::load_rooms(&AREA_1_PATH.to_string()).unwrap());
}

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct CurrentArea {
    pub pool: Arc<HashMap<String, RoomData>>,
}

#[methods]
impl CurrentArea {
    fn new(_owner: &Node2D) -> Self {
        Self {
            pool: AREA_1.clone(),
        }
    }

    #[export]
    fn _ready(&self, owner: &Node2D) {
        let rooms = unsafe {
            ResourceLoader::godot_singleton()
                .load("res://Rooms/Area1/Area1.tscn", "PackedScene", false)
                .unwrap()
                .assume_safe()
        }
        .cast::<PackedScene>()
        .unwrap()
        .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
        .unwrap();

        self.gen_map(owner, rooms);

        unsafe { rooms.assume_unique() }.queue_free();
    }

    fn gen_map(&self, owner: &Node2D, rooms: Ref<Node>) {
        let rooms = unsafe { rooms.assume_safe() };
        let current_rooms = unsafe { owner.get_node("./CurrentRooms").unwrap().assume_safe() };

        current_rooms.get_children().iter().for_each(|c| {
            unsafe { c.to_object::<Node2D>().unwrap().assume_safe() }.queue_free();
        });

        let mut map: Vec<Vec<Option<&RoomData>>> = vec![vec![None; MAP_SIZE]; MAP_SIZE];
        let mut rng = rand::thread_rng();

        for i in 0..27 {
            for j in 0..27 {
                let id = if i == 0 && j == 0 {
                    &"start"
                } else {
                    let pool = Self::gen_pool(i, j, &map);

                    match pool.choose(&mut rng) {
                        Some(id) => id.as_str(),
                        None => continue,
                    }
                };

                match self.pool.get(id) {
                    Some(room_data) => {
                        let room = unsafe {
                            rooms
                                .get_node(format!("./{id}"))
                                .unwrap()
                                .assume_safe()
                                .duplicate(15)
                                .unwrap()
                                .assume_safe()
                        }
                        .cast::<TileMap>()
                        .unwrap();

                        room.set_name(current_rooms.get_children().len().to_string());
                        room.set_position(Vector2::new(
                            TILE_SIZE * GRID_SIZE * i as f32,
                            -TILE_SIZE * GRID_SIZE * j as f32,
                        ));

                        current_rooms.add_child(room, true);

                        map[i][j] = Some(room_data);
                    }
                    None => break,
                }
            }
        }
    }

    fn gen_pool<'a>(i: usize, j: usize, map: &Vec<Vec<Option<&'a RoomData>>>) -> Vec<&'a String> {
        let mut pool = Vec::new();
        let mut count = 0;

        if j < MAP_SIZE {
            if let Some(v) = map.get(i + 1).and_then(|m| match m.get(j) {
                Some(r) => match r {
                    Some(r) => Some(&r.left),
                    None => None,
                },
                None => None,
            }) {
                pool.extend(v);

                count += 1;
            }
        }

        if i > 0 {
            if let Some(v) = map
                .get((i - 1).clamp(0, MAP_SIZE))
                .and_then(|m| match m.get(j) {
                    Some(r) => match r {
                        Some(r) => Some(&r.right),
                        None => None,
                    },
                    None => None,
                })
            {
                pool.extend(v);

                count += 1;
            }
        }

        if j > 0 {
            if let Some(v) = map.get(i).and_then(|m| match m.get(j - 1) {
                Some(r) => match r {
                    Some(r) => Some(&r.top),
                    None => None,
                },
                None => None,
            }) {
                pool.extend(v);

                count += 1;
            }
        }

        if j < MAP_SIZE {
            if let Some(v) = map.get(i).and_then(|m| match m.get(j + 1) {
                Some(r) => match r {
                    Some(r) => Some(&r.bottom),
                    None => None,
                },
                None => None,
            }) {
                pool.extend(v);

                count += 1;
            }
        }

        pool.iter()
            .counts()
            .into_iter()
            .filter_map(|(k, v)| if v == count { Some(*k) } else { None })
            .collect()
    }
}
