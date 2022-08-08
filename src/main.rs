use bevy::prelude::*;

mod game;

use game::GamePlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Kazka".to_string(),
            width: 800.0,
            height: 600.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
