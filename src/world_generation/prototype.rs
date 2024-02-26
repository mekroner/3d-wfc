// This first analizes the tileset, it defines sockets, as well as the which side is up and which
// side is down
use bevy::gltf::Gltf;
use bevy::prelude::*;

use super::dir::{Dir, Rotation};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Socket {
    Nil,
    Sym(u16),
    Asym(u16),
    AsymMir(u16),
    Vert(u16),
}
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

    pub y_rotations: Vec<Rotation>,
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
    let cliff_upper = ass.load("models/terrain/cliff_upper.glb");
    let cliff_upper_corner = ass.load("models/terrain/cliff_upper_corner.glb");
    let ground_prt = Prototype {
        name: "ground",
        asset_handle: ground,
        p_x: Socket::Sym(1),
        n_x: Socket::Sym(1),
        p_y: Socket::Asym(5),
        n_y: Socket::Asym(5),
        p_z: Socket::Sym(1),
        n_z: Socket::Sym(1),
        frequency: 0,
        y_rotations: vec![Rotation::Zero],
    };

    let cliff_low_prt = Prototype {
        name: "cliff_low",
        asset_handle: cliff_low,
        p_x: Socket::Asym(5),
        n_x: Socket::Sym(1),
        p_y: Socket::Vert(2),
        n_y: Socket::Asym(5),
        p_z: Socket::Asym(3),
        n_z: Socket::AsymMir(3),
        frequency: 0,
        y_rotations: vec![Rotation::Zero, Rotation::Half, Rotation::Quarter, Rotation::ThreeQuarter],
    };

    let cliff_low_corner_prt = Prototype {
        name: "cliff_low_corner",
        asset_handle: cliff_low_corner,
        p_x: Socket::Asym(3),
        n_x: Socket::Sym(1),
        p_y: Socket::Vert(3),
        n_y: Socket::Asym(5),
        p_z: Socket::Sym(1),
        n_z: Socket::AsymMir(3),
        frequency: 0,
        y_rotations: vec![Rotation::Zero, Rotation::Half, Rotation::Quarter, Rotation::ThreeQuarter],
    };

    let cliff_upper_prt = Prototype {
        name: "cliff_upper",
        asset_handle: cliff_upper,
        p_x: Socket::Sym(1),
        n_x: Socket::Asym(5),
        p_y: Socket::Asym(5),
        n_y: Socket::Vert(2),
        p_z: Socket::Asym(4),
        n_z: Socket::AsymMir(4),
        frequency: 0,
        y_rotations: vec![Rotation::Zero, Rotation::Half, Rotation::Quarter, Rotation::ThreeQuarter],
    };

    let cliff_upper_corner_prt = Prototype {
        name: "cliff_upper_corner",
        asset_handle: cliff_upper_corner.clone(),
        p_x: Socket::Nil,
        n_x: Socket::Nil,
        p_y: Socket::Nil,
        n_y: Socket::Nil,
        p_z: Socket::Nil,
        n_z: Socket::Nil,
        frequency: 0,
        y_rotations: vec![Rotation::Zero, Rotation::Half, Rotation::Quarter, Rotation::ThreeQuarter],
    };

    let nil_prt = Prototype {
        name: "cliff_upper_corner",
        asset_handle: cliff_upper_corner,
        p_x: Socket::AsymMir(5),
        n_x: Socket::AsymMir(5),
        p_y: Socket::AsymMir(5),
        n_y: Socket::AsymMir(5),
        p_z: Socket::AsymMir(5),
        n_z: Socket::AsymMir(5),
        frequency: 0,
        y_rotations: vec![Rotation::Zero],
    };

    let assets = vec![
        ground_prt,
        cliff_low_prt,
        cliff_upper_prt,
        cliff_low_corner_prt,
        cliff_upper_corner_prt,
        nil_prt,
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
        if ass.get_load_state(&prt.asset_handle).is_none() {
            debug!("Not all prototypes loaded!");
            return;
        }
    }
    debug!("Prototypes loaded!");
    next_state.set(PrototypesLoadState::Finished);
}
