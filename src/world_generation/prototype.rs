// This first analizes the tileset, it defines sockets, as well as the which side is up and which
// side is down

use bevy::prelude::*;
use bevy::gltf::Gltf;

// use super::tile::Tile;

pub enum Rotation {
    Zero,
    Quarter,
    Half,
    ThreeQuarter
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Socket(pub u32);

pub struct Prototype {
    pub name: &'static str,
    pub asset_handle: Handle<Gltf>,
    pub p_x: Socket,
    pub n_x: Socket,
    pub p_y: Socket,
    pub n_y: Socket,
    pub p_z: Socket,
    pub n_z: Socket,

    pub frequency: u32,
}


