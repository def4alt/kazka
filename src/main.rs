use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod game;
mod player;

use game::GamePlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Kazka".to_string(),
            width: 800.0,
            height: 600.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(GamePlugin)
        .run();
}
