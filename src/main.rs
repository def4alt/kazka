use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
       app.add_startup_system(add_people).add_system(greet_people); 
    }
}

fn main() {
    App::new().add_plugins(DefaultPlugins).add_plugin(HelloPlugin).run();
}

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("John Weak".to_string()));
    commands.spawn().insert(Person).insert(Name("Rebecka Holmes".to_string()));
    commands.spawn().insert(Person).insert(Name("Sam Topkins".to_string()));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("Hello {}!", name.0);
    }
}
