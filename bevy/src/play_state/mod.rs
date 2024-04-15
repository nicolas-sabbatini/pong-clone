use bevy::{math::bounding::Aabb2d, prelude::*, sprite::Mesh2dHandle};

use crate::{
    asset_loading::AssetList,
    flow_control::{GameState, PlayState},
};

use self::{
    ball::{BALL_HEIGHT, BALL_WIDTH},
    paddle::{PADDLE_HEIGHT, PADDLE_WIDTH},
};

mod ball;
mod paddle;

#[derive(Resource)]
struct TextConfig {
    font: Handle<Font>,
}

#[derive(Resource)]
struct PaddleSprite {
    mesh: Mesh2dHandle,
    material: Handle<ColorMaterial>,
}

#[derive(Resource)]
struct BallSprite {
    mesh: Mesh2dHandle,
    material: Handle<ColorMaterial>,
}

#[derive(Component)]
struct HitBox {
    poligon: Rectangle,
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets)
            .add_systems(OnEnter(GameState::RunMainLoop), start_game)
            .add_systems(Update, draw_coliders);

        app.add_plugins((paddle::Plug, ball::Plug));
    }
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_list: ResMut<AssetList>,
) {
    let font = asset_server.load("Bubbly_Bold.ttf");
    asset_list.0.push(font.clone().untyped());
    commands.insert_resource(TextConfig { font });

    let paddle_mesh = Mesh2dHandle(meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT)));
    let paddle_material = materials.add(Color::rgb(1.0, 1.0, 1.0));
    commands.insert_resource(PaddleSprite {
        mesh: paddle_mesh,
        material: paddle_material,
    });

    let ball_mesh = Mesh2dHandle(meshes.add(Rectangle::new(BALL_WIDTH, BALL_HEIGHT)));
    let ball_material = materials.add(Color::rgb(1.0, 1.0, 1.0));
    commands.insert_resource(BallSprite {
        mesh: ball_mesh,
        material: ball_material,
    });
}

fn start_game(mut next_state: ResMut<NextState<PlayState>>) {
    // Change state to game start
    next_state.set(PlayState::Match);
}

fn draw_coliders(mut gizmos: Gizmos, query: Query<(&HitBox, &GlobalTransform)>) {
    let color = Color::RED;
    for (hit_box, transform) in query.iter() {
        let (_, rotation, translation) = transform.to_scale_rotation_translation();
        gizmos.primitive_2d(
            hit_box.poligon,
            translation.xy(),
            rotation.to_euler(EulerRot::YXZ).2,
            color,
        );
    }
}
