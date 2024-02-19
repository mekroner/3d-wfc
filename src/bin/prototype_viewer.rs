use std::f32::consts::PI;

use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::HashMap;

use rand::random;

use utg::fly_camera::FlyCamPlugin;
use utg::world_generation::{grid_gizmo, prototype::*, TILE_SIZE};

fn main() {
    use PrototypesLoadState as PLS;
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Prototype View Tool".into(),
                        resolution: (800., 600.).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(bevy::log::LogPlugin {
                    level: bevy::log::Level::DEBUG,
                    ..default()
                }),
        )
        .insert_resource(SocketColors(HashMap::new()))
        .add_plugins(FlyCamPlugin)
        .add_state::<PLS>()
        .add_systems(Startup, spawn_light)
        .add_systems(OnEnter(PLS::Loading), load_prototypes)
        .add_systems(
            Update,
            check_prototypes_loaded.run_if(in_state(PLS::Loading)),
        )
        .add_systems(
            OnEnter(PLS::Finished),
            (spawn_prototypes_in_grid, determine_socket_color),
        )
        .add_systems(Update, compass_gizmo)
        .add_systems(
            Update,
            display_colored_sockets.run_if(in_state(PLS::Finished)),
        )
        .run();
}

fn spawn_light(mut commands: Commands) {
    let trnsfrm = Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1000.0,
            range: 100.0,
            ..default()
        },
        transform: trnsfrm,
        ..default()
    });
}

fn generate_rules(prts: Res<Prototypes>) {
}

#[derive(Resource)]
struct Prototypes(Vec<Prototype>);

fn load_prototypes(mut cmds: Commands, ass: Res<AssetServer>) {
    let ground = ass.load("models/terrain/ground.glb");
    let cliff_low = ass.load("models/terrain/cliff_low.glb");
    let cliff_low_corner = ass.load("models/terrain/cliff_low_corner.glb");
    let cliff_upper = ass.load("models/terrain/cliff_upper.glb");
    let cliff_upper_corner = ass.load("models/terrain/cliff_upper_corner.glb");
    let ground_prt = Prototype {
        name: "ground",
        asset_handle: ground,
        p_x: Socket(1),
        n_x: Socket(1),
        p_y: Socket(0),
        n_y: Socket(0),
        p_z: Socket(1),
        n_z: Socket(1),
        frequency: 0,
    };

    let cliff_low_prt = Prototype {
        name: "cliff_low",
        asset_handle: cliff_low,
        p_x: Socket(0),
        n_x: Socket(1),
        p_y: Socket(2),
        n_y: Socket(0),
        p_z: Socket(3),
        n_z: Socket(3),
        frequency: 0,
    };

    let cliff_low_corner_prt = Prototype {
        name: "cliff_low_corner",
        asset_handle: cliff_low_corner,
        p_x: Socket(0),
        n_x: Socket(1),
        p_y: Socket(0),
        n_y: Socket(0),
        p_z: Socket(1),
        n_z: Socket(3),
        frequency: 0,
    };

    let cliff_upper_prt = Prototype {
        name: "cliff_upper",
        asset_handle: cliff_upper,
        p_x: Socket(1),
        n_x: Socket(0),
        p_y: Socket(0),
        n_y: Socket(2),
        p_z: Socket(0),
        n_z: Socket(0),
        frequency: 0,
    };

    let cliff_upper_corner_prt = Prototype {
        name: "cliff_upper_corner",
        asset_handle: cliff_upper_corner,
        p_x: Socket(0),
        n_x: Socket(0),
        p_y: Socket(0),
        n_y: Socket(0),
        p_z: Socket(0),
        n_z: Socket(0),
        frequency: 0,
    };

    let assets = vec![
        ground_prt,
        cliff_low_prt,
        cliff_low_corner_prt,
        cliff_upper_prt,
        cliff_upper_corner_prt,
    ];
    cmds.insert_resource(Prototypes(assets));
}

#[derive(States, Default, Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum PrototypesLoadState {
    #[default]
    Loading,
    Finished,
}

fn check_prototypes_loaded(
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

// FIXME: This should spawn a grid, but it spawns them in a line
fn spawn_prototypes_in_grid(
    mut cmds: Commands,
    prts: Res<Prototypes>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    // let grid_dim = (prts.0.len() as f32).sqrt().floor().into();
    for (i, prt) in prts.0.iter().enumerate() {
        let pos = Vec3::new(i as f32 * TILE_SIZE * 2., 0.5, 0.);
        let handle = &prt.asset_handle;
        let gltf = assets_gltf.get(handle).expect("Asset should be loaded");
        let transform = Transform {
            translation: pos,
            ..default()
        };
        let _entity = cmds.spawn(SceneBundle {
            scene: gltf.scenes[0].clone(),
            transform,
            ..default()
        });
    }
}

#[derive(Resource)]
struct SocketColors(HashMap<Socket, Color>);

fn determine_socket_color(prts: Res<Prototypes>, mut colors: ResMut<SocketColors>) {
    for prt in prts.0.iter() {
        if !colors.0.contains_key(&prt.p_x) {
            colors.0.insert(prt.p_x, random_color());
        }
        if !colors.0.contains_key(&prt.n_x) {
            colors.0.insert(prt.n_x, random_color());
        }
        if !colors.0.contains_key(&prt.p_y) {
            colors.0.insert(prt.p_y, random_color());
        }
        if !colors.0.contains_key(&prt.n_y) {
            colors.0.insert(prt.n_y, random_color());
        }
        if !colors.0.contains_key(&prt.p_z) {
            colors.0.insert(prt.p_z, random_color());
        }
        if !colors.0.contains_key(&prt.n_z) {
            colors.0.insert(prt.n_z, random_color());
        }
    }
}

fn random_color() -> Color {
    Color::Rgba{
        red: random(),
        green: random(),
        blue: random(),
        alpha: 1.0,
    }
}

fn display_colored_sockets(mut gizmos: Gizmos, prts: Res<Prototypes>, colors: Res<SocketColors>) {
    for (i, prt) in prts.0.iter().enumerate() {
        let pos = Vec3::new(i as f32 * TILE_SIZE * 2., 0.5, 0.);

        // positive Y
        gizmos.rect(
            pos + Vec3::Y * 0.5,
            Quat::from_rotation_x(PI / 2.0),
            Vec2::splat(TILE_SIZE as f32 * 0.9),
            colors.0[&prt.p_y],
        );

        // negative Y
        gizmos.rect(
            pos - Vec3::Y * 0.5,
            Quat::from_rotation_x(PI / 2.0),
            Vec2::splat(TILE_SIZE as f32 * 0.9),
            colors.0[&prt.n_y],
        );

        // positive X
        gizmos.rect(
            pos + Vec3::X * 0.5,
            Quat::from_rotation_y(PI / 2.0),
            Vec2::splat(TILE_SIZE as f32 * 0.9),
            colors.0[&prt.p_x],
        );

        // negative X
        gizmos.rect(
            pos - Vec3::X * 0.5,
            Quat::from_rotation_y(PI / 2.0),
            Vec2::splat(TILE_SIZE as f32 * 0.9),
            colors.0[&prt.n_x],
        );

        // positive Z
        gizmos.rect(
            pos + Vec3::Z * 0.5,
            Quat::from_rotation_x(0.0),
            Vec2::splat(TILE_SIZE as f32 * 0.9),
            colors.0[&prt.p_z],
        );

        // negative Z
        gizmos.rect(
            pos - Vec3::Z * 0.5,
            Quat::from_rotation_x(0.0),
            Vec2::splat(TILE_SIZE as f32 * 0.9),
            colors.0[&prt.n_z],
        );
    }
}

pub fn compass_gizmo(mut gizmos: Gizmos) {
    gizmos.ray(Vec3::ZERO, Vec3::Y, Color::BLUE);
    gizmos.ray(Vec3::ZERO, Vec3::X, Color::RED);
    gizmos.ray(Vec3::ZERO, Vec3::Z, Color::GREEN);
}
