use bevy::color::palettes::css::{BLACK, BLUE, LIGHT_YELLOW};
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use bevy::pbr::ExtendedMaterial;
use bevy::prelude::*;
use bevy_wind_waker_shader::prelude::*;
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
        .add_plugins(WindWakerShaderPlugin::default())
        .add_plugins(FreeCameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut extended_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, WindWakerShader>>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.0).mesh().uv(32, 18))),
        Transform::from_xyz(0.0, 1.0, 0.0),
        MeshMaterial3d(
            extended_materials.add(ExtendedMaterial::<StandardMaterial, WindWakerShader> {
                base: StandardMaterial {
                    base_color: Color::from(BLUE), // 基础材质属性
                    ..default()
                },
                extension: WindWakerShaderBuilder::default()
                    .override_rim_color(Color::from(BLACK))
                    .build(),
            }),
        ),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(extended_materials.add(ExtendedMaterial::<
            StandardMaterial,
            WindWakerShader,
        > {
            base: StandardMaterial {
                base_color: Color::from(LIGHT_YELLOW), // 基础材质属性
                ..default()
            },
            extension: WindWakerShaderBuilder::default().build(),
        })),
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_translation(Vec3::new(1.0, 1.0, 0.0)).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
