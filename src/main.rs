use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod errors;
mod player;
mod cursor;
mod queries;
mod views;
mod state;

use state::ConnectionState;
use views::ViewState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin{
                primary_window: Some(Window {
                    title: String::from(
                        "Tinker",
                    ),
                    ..Default::default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()))

        .init_resource::<ConnectionState>()

        .add_plugins(TilemapPlugin)
        .add_plugins(TiledMapPlugin::default())

        .init_state::<ViewState>()
        .add_plugins(views::menu::main_menu)
        .add_plugins(views::game::main_game)

        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    // mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);
}