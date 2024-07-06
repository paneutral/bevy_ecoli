use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 80.;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(20., CAMERA_DISTANCE, 20.).looking_at(Vec3::ZERO, Vec3::Z),
            ..default()
        },

    ));
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: false,
            intensity: 100_000_000.,
            range: 200.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        transform: Transform::from_xyz(-50., 50.0, 50.),
        ..default()
    });
}
