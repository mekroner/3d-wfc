use std::f32::consts::PI;

use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::HashMap;
use strum::IntoEnumIterator;

use utg::fly_camera::FlyCam;
use utg::fly_camera::FlyCamPlugin;
use utg::world_generation::prototype::*;
use utg::world_generation::tile::*;
use utg::world_generation::TILE_SIZE;

const DISPLAY_AREA_SIZE: f32 = 3. * TILE_SIZE;

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
                    // level: bevy::log::Level::DEBUG,
                    ..default()
                }),
        )
        .add_plugins(FlyCamPlugin)
        .add_state::<PLS>()
        .insert_resource(Tiles(HashMap::new()))
        .insert_resource(AdjRuleSet(HashMap::new()))
        .add_systems(Startup, spawn_light)
        .add_systems(OnEnter(PLS::Loading), load_prototypes)
        .add_systems(
            Update,
            check_prototypes_loaded.run_if(in_state(PLS::Loading)),
        )
        .add_systems(OnEnter(PLS::Finished), generate_tiles_and_rules)
        .add_systems(
            OnEnter(PLS::Finished),
            spawn_rule_examples.after(generate_tiles_and_rules),
        )
        .add_systems(Update, tie_light_to_cam)
        .add_systems(Update, grid_gizmo)
        .run();
}

#[derive(Component)]
struct Light;

fn spawn_light(mut commands: Commands) {
    let trnsfrm = Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn((PointLightBundle {
        point_light: PointLight {
            intensity: 1000.0,
            range: 100.0,
            ..default()
        },
        transform: trnsfrm,
        ..default()
    }, Light));
}

fn tie_light_to_cam(
    mut light: Query<&mut Transform, (With<Light>, Without<FlyCam>)>,
    query: Query<&Transform, With<FlyCam>>,
) {
    let trfm = query.single();
    let mut light_trfm = light.single_mut();
    light_trfm.translation = trfm.translation.clone();
}

// FIXME: This should spawn a grid, but it spawns them in a line
fn spawn_rule_examples(
    mut cmds: Commands,
    rule_set: Res<AdjRuleSet>,
    tiles: Res<Tiles>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    // let grid_dim = (prts.0.len() as f32).sqrt().floor().into();
    let mut index = 0;
    for (id, rule) in rule_set.0.iter() {
        let tile = tiles.0.get(id).unwrap();
        let handle = &tile.asset_handle;
        let gltf = assets_gltf.get(handle).expect("Asset should be loaded");

        for dir in Dir::iter() {
            for other_id in rule.from_dir(dir) {
                let pos = Vec3::new(index as f32 * DISPLAY_AREA_SIZE, 0.5, 0.);
                let other_pos = pos + dir.to_vec3() * TILE_SIZE;
                let other_tile = tiles.0.get(other_id).unwrap();
                let other_handle = &other_tile.asset_handle;
                let other_gltf = assets_gltf
                    .get(other_handle)
                    .expect("Asset should be loaded");
                let transform = Transform {
                    translation: pos,
                    ..default()
                };
                let other_transform = Transform {
                    translation: other_pos,
                    ..default()
                };
                let _entity = cmds.spawn(SceneBundle {
                    scene: gltf.scenes[0].clone(),
                    transform,
                    ..default()
                });
                let _other_entity = cmds.spawn(SceneBundle {
                    scene: other_gltf.scenes[0].clone(),
                    transform: other_transform,
                    ..default()
                });
                index += 1;
            }
        }
    }
}

fn grid_gizmo(mut gizmos: Gizmos) {
    for z in -50..50 {
        for x in -50..50 {
            gizmos.rect(
                Vec3::new(x as f32, 0.0, z as f32),
                Quat::from_rotation_x(PI / 2.0),
                Vec2::splat(TILE_SIZE as f32),
                Color::YELLOW,
            )
        }
    }
}
