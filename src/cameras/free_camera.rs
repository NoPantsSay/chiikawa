// 在你的 camera 插件模块中
use bevy::camera::Exposure;
use bevy::camera_controller::free_camera;
use bevy::prelude::*;

pub struct FreeCameraPlugin {
    pub initial_transform: Transform,
}

impl Default for FreeCameraPlugin {
    fn default() -> Self {
        Self {
            initial_transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        }
    }
}

impl Plugin for FreeCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(free_camera::FreeCameraPlugin);
        app.insert_resource(CameraInitialTransform(self.initial_transform));
        app.add_systems(Startup, spawn_camera);
    }
}

#[derive(Resource)]
struct CameraInitialTransform(Transform);

fn spawn_camera(mut commands: Commands, initial_transform: Res<CameraInitialTransform>) {
    commands.spawn((
        Camera3d::default(),
        initial_transform.0,
        free_camera::FreeCamera {
            sensitivity: 0.2,
            friction: 25.0,
            walk_speed: 3.0,
            run_speed: 9.0,
            ..default()
        },
        Exposure::BLENDER,
    ));
}
