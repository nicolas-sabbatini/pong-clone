#![allow(clippy::wildcard_imports)]
use bevy::{prelude::*, window::WindowResolution};
use config::*;

mod asset_loading;
mod camera;
mod config;
mod constants;
mod flow_control;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(GAME_WIDTH * 2.0, GAME_HEIGHT * 2.0),
                    title: WINDOW_TITLE.to_string(),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .insert_resource(Msaa::Off);

    app.add_plugins((camera::Plug, flow_control::Plug, asset_loading::Plug));

    app.run();
}
