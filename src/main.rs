use bevy::color::palettes::basic::*;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use bevy::prelude::*;
use chiikawa::cameras::free_camera::FreeCameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                frame_time_graph_config: FrameTimeGraphConfig {
                    enabled: false,
                    ..default()
                },
                ..default()
            },
        })
        .add_plugins(FreeCameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.0).mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(Color::from(BLUE))),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_translation(Vec3::new(1.0, 1.0, 0.0)).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
