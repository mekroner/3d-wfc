use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    math::quat,
    prelude::*,
    window,
};

const CAMERA_HEIGHT: f32 = 25.;
const CAMERA_SPEED:f32 = 2.;
pub struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, input_handler)
            .add_systems(Update, camera_zoom);
    }
}

#[derive(Component)]
struct PanOrbitCamera {
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        Self {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

fn input_handler(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<PanOrbitCamera>>,
    time: Res<Time>,
) {
    let mut pan = Vec3::ZERO;

    if input.pressed(KeyCode::W) {
        pan.x = -1.0;
    }
    if input.pressed(KeyCode::S) {
        pan.x = 1.0;
    }
    if input.pressed(KeyCode::A) {
        pan.z = 1.0;
    }
    if input.pressed(KeyCode::D) {
        pan.z = -1.0;
    }

    for mut transform in query.iter_mut() {
        transform.translation += CAMERA_SPEED * pan * time.delta_seconds();
    }
}

fn camera_zoom(
    mut ev_scroll: EventReader<MouseWheel>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    let mut scroll = 0.0;

    for ev in ev_scroll.read() {
        scroll += ev.y;
    }

    for (mut pan_orbit, mut tfm) in query.iter_mut() {
        if scroll.abs() > 0.0 {
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
            let rot_matrix = Mat3::from_quat(tfm.rotation);
            tfm.translation =
                pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }

    }
}

fn spawn_camera(mut cmds: Commands) {
    let translation = Vec3::new(0.0, CAMERA_HEIGHT, 0.0);
    let radius = translation.length();

    cmds.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera {
            radius,
            ..default()
        },
    ));
}
