#![allow(
    clippy::needless_pass_by_value,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
use bevy::{
    prelude::*,
    render::{
        camera::{RenderTarget, ScalingMode},
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
        view::RenderLayers,
    },
};

use crate::config::{
    GAME_CAMERA_CLEAR_COLOR, GAME_CAMERA_NAME, GAME_CAMERA_TARGET_NAME, GAME_HEIGHT, GAME_WIDTH,
    WINDOW_CAMERA_CLEAR_COLOR, WINDOW_CAMERA_NAME,
};

const BGRA_PIXEL_SIZE: usize = 4;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Component)]
pub struct GameCamera;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Component)]
pub struct WindowCamera;

#[derive(Debug, Component)]
pub struct TextureRenderTarget;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // Set up windows camera
    let mut windows_camera = Camera2dBundle::default();
    windows_camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: GAME_WIDTH,
        min_height: GAME_HEIGHT,
    };
    windows_camera.camera.clear_color = ClearColorConfig::Custom(WINDOW_CAMERA_CLEAR_COLOR);
    windows_camera.camera.order = 99;
    commands.spawn((
        windows_camera,
        Name::new(WINDOW_CAMERA_NAME),
        // Only see layer 1
        RenderLayers::layer(1),
        WindowCamera,
    ));

    // Set up letter boxing
    // Create render target texture
    let render_target_size = Extent3d {
        width: GAME_WIDTH as u32,
        height: GAME_HEIGHT as u32,
        ..default()
    };
    let mut render_target_image = Image::new_fill(
        render_target_size,
        TextureDimension::D2,
        &vec![255; (GAME_WIDTH * GAME_HEIGHT) as usize * BGRA_PIXEL_SIZE],
        #[cfg(not(target_arch = "wasm32"))]
        TextureFormat::Bgra8UnormSrgb,
        // Wasm neds a diferent Texture format
        #[cfg(target_arch = "wasm32")]
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    // By default an image can't be used as a render target so we need to setup the render target falg
    render_target_image.texture_descriptor.usage |= TextureUsages::RENDER_ATTACHMENT;
    // Add the render target to the image assets
    let render_target_handle = images.add(render_target_image);
    // Spawn render target on the world
    commands.spawn((
        SpriteBundle {
            texture: render_target_handle.clone(),
            ..Default::default()
        },
        Name::new(GAME_CAMERA_TARGET_NAME),
        // Only windows camera can see
        RenderLayers::layer(1),
        TextureRenderTarget,
    ));

    // Set up game camera
    let mut game_camera = Camera2dBundle::default();
    // Set up the render target created previously as target
    game_camera.camera.target = RenderTarget::Image(render_target_handle);
    game_camera.camera.clear_color = ClearColorConfig::Custom(GAME_CAMERA_CLEAR_COLOR);
    game_camera.camera.order = 1;
    commands.spawn((game_camera, Name::new(GAME_CAMERA_NAME), GameCamera));
}
