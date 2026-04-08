use bevy::camera_controller::free_camera;
use bevy::prelude::*;

pub struct FreeCameraPlugin;
impl Plugin for FreeCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(free_camera::FreeCameraPlugin);
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        // Free camera controller settings
        free_camera::FreeCamera {
            sensitivity: 0.2,
            friction: 25.0,
            walk_speed: 3.0,
            run_speed: 9.0,
            ..default()
        },
    ));
}
