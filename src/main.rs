use bevy::color::palettes::css::{BLUE, LIGHT_BLUE, WHITE};
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use bevy::prelude::*;
use bevy_mod_outline::*;
use bevy_wind_waker_shader::prelude::*;
use chiikawa::cameras::free_camera::FreeCameraPlugin;
use fast_poisson::Poisson2D;

#[derive(Component)]
struct Grass;

fn main() {
    App::new()
        .insert_resource(GlobalAmbientLight {
            brightness: 0.0,
            ..default()
        })
        .insert_resource(ClearColor(Color::from(LIGHT_BLUE)))
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
        .add_plugins(OutlinePlugin)
        .add_plugins(FreeCameraPlugin::default())
        .add_systems(Startup, (setup, setup_grass))
        .add_systems(Update, rotation_grass)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.0).mesh().uv(32, 18))),
        Transform::from_xyz(0.0, 1.0, 0.0),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::from(BLUE), // 基础材质属性
            ..default()
        })),
        WindWakerShaderBuilder::default()
            .override_highlight_color(WHITE.into())
            .override_shadow_color(WHITE.into())
            .build(),
        OutlineVolume {
            visible: true,
            colour: Color::srgb(0.0, 0.0, 0.0),
            width: 5.0,
        },
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.97, 0.96, 0.90), // 基础材质属性
            ..default()
        })),
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn setup_grass(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let grass_texture_handle = asset_server.load("grass0.png");
    let grass_mesh = meshes.add(Plane3d::default().mesh().size(1.0, 1.0));
    let grass_material = materials.add(StandardMaterial {
        base_color_texture: Some(grass_texture_handle),
        base_color: Color::WHITE,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    let points: Vec<[f64; 2]> = Poisson2D::new().with_dimensions([8.0, 8.0], 2.0).generate();

    for &[x, z] in points.iter().take(10) {
        let x = x as f32 - 4.0;
        let z = z as f32 - 4.0;

        commands.spawn((
            Mesh3d(grass_mesh.clone()),
            MeshMaterial3d(grass_material.clone()),
            Transform::from_xyz(x, 0.001, z),
            Grass,
        ));
    }
}

fn rotation_grass(
    mut grass_query: Query<&mut Transform, With<Grass>>,
    camera_query: Query<&Transform, (With<Camera>, Without<Grass>)>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    let camera_yaw = camera_transform.rotation.to_euler(EulerRot::YXZ).0;

    for mut transform in &mut grass_query {
        let (_, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        transform.rotation = Quat::from_euler(EulerRot::YXZ, camera_yaw, pitch, roll);
    }
}
