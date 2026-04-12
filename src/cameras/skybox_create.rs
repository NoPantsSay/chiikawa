use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

/// 创建一个纯蓝色的立方体贴图纹理
pub fn create_cubemap(images: &mut ResMut<Assets<Image>>, color: Srgba) -> Handle<Image> {
    // 定义立方体贴图的大小（建议使用 2 的幂，如 32x32）
    let size = 32;

    // 每个面的像素数据：6 个面 × (size × size) 个像素 × 4 个通道 (RGBA)
    let pixel_count = 6 * size * size;
    let mut pixels = vec![0u8; pixel_count * 4];

    for pixel in pixels.chunks_exact_mut(4) {
        pixel[0] = (color.red * 255.0) as u8; // R
        pixel[1] = (color.green * 255.0) as u8; // G
        pixel[2] = (color.blue * 255.0) as u8; // B
        pixel[3] = (color.alpha * 255.0) as u8; // A
    }

    // 创建图像，标记为立方体贴图
    let mut image = Image::new_fill(
        Extent3d {
            width: size as u32,
            height: size as u32,
            depth_or_array_layers: 6, // 6 个面 = 立方体贴图
        },
        TextureDimension::D2,
        &pixels,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );

    // 标记为立方体贴图纹理视图
    image.texture_view_descriptor = Some(bevy::render::render_resource::TextureViewDescriptor {
        dimension: Some(bevy::render::render_resource::TextureViewDimension::Cube),
        ..default()
    });

    images.add(image)
}
