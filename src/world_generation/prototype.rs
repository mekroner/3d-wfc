use std::ops::Range;

// This first analizes the tileset, it defines sockets, as well as the which side is up and which
// side is down
use bevy::gltf::Gltf;
use bevy::prelude::*;

use super::{
    dir::{Dir, Rotation},
    CHUNK_HIGHT,
};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Socket {
    Ground,
    Air,
    Sym(u16),
    Asym(u16),
    AsymMir(u16),
    Vert(u16),
}
pub struct Prototype {
    pub name: &'static str,
    pub asset_handle: Option<Handle<Gltf>>,
    pub p_x: Socket,
    pub n_x: Socket,
    pub p_y: Socket,
    pub n_y: Socket,
    pub p_z: Socket,
    pub n_z: Socket,

    pub weight: usize,

    pub y_rotations: Vec<Rotation>,
    pub y_level: Option<Range<usize>>,
}

impl Prototype {
    pub fn socket_from_dir(&self, dir: Dir) -> Socket {
        match dir {
            Dir::Forward => self.n_z,
            Dir::Backward => self.p_z,
            Dir::Left => self.n_x,
            Dir::Right => self.p_x,
            Dir::Up => self.p_y,
            Dir::Down => self.n_y,
        }
    }
}

#[derive(Resource)]
pub struct Prototypes(pub Vec<Prototype>);

// FIXME: This can be done using a new asset type
pub fn load_prototypes(mut cmds: Commands, ass: Res<AssetServer>) {
    let ground = ass.load("models/terrain/ground.glb");
    let cliff_low = ass.load("models/terrain/cliff_low.glb");
    let cliff_low_corner = ass.load("models/terrain/cliff_low_corner.glb");
    let cliff_low_corner2 = ass.load("models/terrain/cliff_low_corner2.glb");
    let cliff_upper = ass.load("models/terrain/cliff_upper.glb");
    let cliff_upper_corner = ass.load("models/terrain/cliff_upper_corner.glb");
    let cliff_upper_corner2 = ass.load("models/terrain/cliff_upper_corner2.glb");
    let ground_prt = Prototype {
        name: "ground",
        asset_handle: Some(ground),
        p_x: Socket::Sym(1),
        n_x: Socket::Sym(1),
        p_y: Socket::Air,
        n_y: Socket::Ground,
        p_z: Socket::Sym(1),
        n_z: Socket::Sym(1),
        weight: 25,
        y_rotations: vec![Rotation::Zero],
        y_level: None,
    };

    let cliff_low_prt = Prototype {
        name: "cliff_low",
        asset_handle: Some(cliff_low),
        p_x: Socket::Ground,
        n_x: Socket::Sym(1),
        p_y: Socket::Vert(2),
        n_y: Socket::Ground,
        p_z: Socket::Asym(3),
        n_z: Socket::AsymMir(3),
        weight: 1,
        y_rotations: vec![
            Rotation::Zero,
            Rotation::Half,
            Rotation::Quarter,
            Rotation::ThreeQuarter,
        ],
        y_level: Some(0..(CHUNK_HIGHT)),
    };

    let cliff_low_corner_prt = Prototype {
        name: "cliff_low_corner",
        asset_handle: Some(cliff_low_corner),
        p_x: Socket::Asym(3),
        n_x: Socket::Sym(1),
        p_y: Socket::Vert(3),
        n_y: Socket::Ground,
        p_z: Socket::Sym(1),
        n_z: Socket::AsymMir(3),
        weight: 1,
        y_rotations: vec![
            Rotation::Zero,
            Rotation::Half,
            Rotation::Quarter,
            Rotation::ThreeQuarter,
        ],
        y_level: Some(0..(CHUNK_HIGHT)),
    };

    let cliff_low_corner2_prt = Prototype {
        name: "cliff_low_corner2",
        asset_handle: Some(cliff_low_corner2),
        p_x: Socket::Ground,
        n_x: Socket::AsymMir(3),
        p_y: Socket::Vert(4),
        n_y: Socket::Ground,
        p_z: Socket::Asym(3),
        n_z: Socket::Ground,
        weight: 1,
        y_rotations: vec![
            Rotation::Zero,
            Rotation::Half,
            Rotation::Quarter,
            Rotation::ThreeQuarter,
        ],
        y_level: Some(0..(CHUNK_HIGHT)),
    };

    let cliff_upper_prt = Prototype {
        name: "cliff_upper",
        asset_handle: Some(cliff_upper),
        p_x: Socket::Sym(1),
        n_x: Socket::Air,
        p_y: Socket::Air,
        n_y: Socket::Vert(2),
        p_z: Socket::Asym(4),
        n_z: Socket::AsymMir(4),
        weight: 1,
        y_rotations: vec![
            Rotation::Zero,
            Rotation::Half,
            Rotation::Quarter,
            Rotation::ThreeQuarter,
        ],
        y_level: Some(1..(CHUNK_HIGHT + 1)),
    };

    let cliff_upper_corner_prt = Prototype {
        name: "cliff_upper_corner",
        asset_handle: Some(cliff_upper_corner.clone()),
        p_x: Socket::Asym(4),
        n_x: Socket::Air,
        p_y: Socket::Air,
        n_y: Socket::Vert(3),
        p_z: Socket::Air,
        n_z: Socket::AsymMir(4),
        weight: 1,
        y_rotations: vec![
            Rotation::Zero,
            Rotation::Half,
            Rotation::Quarter,
            Rotation::ThreeQuarter,
        ],
        y_level: Some(1..(CHUNK_HIGHT + 1)),
    };

    let cliff_upper_corner2_prt = Prototype {
        name: "cliff_upper_corner2",
        asset_handle: Some(cliff_upper_corner2.clone()),
        p_x: Socket::Sym(1),
        n_x: Socket::AsymMir(4),
        p_y: Socket::Air,
        n_y: Socket::Vert(4),
        p_z: Socket::Asym(4),
        n_z: Socket::Sym(1),
        weight: 1,
        y_rotations: vec![
            Rotation::Zero,
            Rotation::Half,
            Rotation::Quarter,
            Rotation::ThreeQuarter,
        ],
        y_level: Some(1..(CHUNK_HIGHT + 1)),
    };

    let air_prt = Prototype {
        name: "air",
        asset_handle: None,
        p_x: Socket::Air,
        n_x: Socket::Air,
        p_y: Socket::Air,
        n_y: Socket::Air,
        p_z: Socket::Air,
        n_z: Socket::Air,
        weight: 4,
        y_rotations: vec![Rotation::Zero],
        y_level: Some(1..(CHUNK_HIGHT + 1)),
    };

    let dirt_prt = Prototype {
        name: "dirt",
        asset_handle: None,
        p_x: Socket::Ground,
        n_x: Socket::Ground,
        p_y: Socket::Ground,
        n_y: Socket::Ground,
        p_z: Socket::Ground,
        n_z: Socket::Ground,
        weight: 4,
        y_rotations: vec![Rotation::Zero],
        y_level: Some(0..(CHUNK_HIGHT)),
    };

    let assets = vec![
        ground_prt,
        cliff_low_prt,
        cliff_low_corner_prt,
        cliff_low_corner2_prt,
        cliff_upper_prt,
        cliff_upper_corner_prt,
        cliff_upper_corner2_prt,
        air_prt,
        dirt_prt,
    ];
    cmds.insert_resource(Prototypes(assets));
}

#[derive(States, Default, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PrototypesLoadState {
    #[default]
    Loading,
    Finished,
}

pub fn check_prototypes_loaded(
    ass: Res<AssetServer>,
    prts: Res<Prototypes>,
    mut next_state: ResMut<NextState<PrototypesLoadState>>,
) {
    for prt in &prts.0 {
        let Some(ref handle) = prt.asset_handle else {
            continue;
        };
        if ass.get_load_state(handle).is_none() {
            debug!("Not all prototypes loaded!");
            return;
        }
    }
    debug!("Prototypes loaded!");
    next_state.set(PrototypesLoadState::Finished);
}
