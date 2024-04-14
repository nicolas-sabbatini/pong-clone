// https://github.com/maciekglowka/hike_deck/blob/main/src/assets.rs
use bevy::asset::LoadState;
use bevy::prelude::*;

use crate::flow_control::GameState;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetList>()
            .add_systems(
                Update,
                (check_asset_loading).run_if(in_state(GameState::LoadAssets)),
            )
            .add_systems(OnExit(GameState::LoadAssets), clear_asset_list);
    }
}

#[derive(Default, Resource)]
pub struct AssetList(pub Vec<UntypedHandle>);

pub fn check_asset_loading(
    asset_server: Res<AssetServer>,
    asset_list: Res<AssetList>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let status = asset_list.0.iter().map(UntypedHandle::id).fold(
        LoadState::Loaded,
        |general_status, asset_id| {
            let status = asset_server
                .get_load_state(asset_id)
                .expect("The asset do not exist");
            match status {
                LoadState::Failed => LoadState::Failed,
                LoadState::Loaded if general_status != LoadState::Loaded => general_status,
                LoadState::Loaded => LoadState::Loaded,
                _ => LoadState::Loading,
            }
        },
    );
    match status {
        LoadState::Loaded => {
            next_state.set(GameState::RunMainLoop);
        }
        LoadState::Failed => {
            panic!("Asset loading error!");
        }
        _ => {}
    };
}

fn clear_asset_list(mut asset_server: ResMut<AssetList>) {
    asset_server.0.clear();
}
