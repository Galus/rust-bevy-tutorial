use bevy::prelude::*;

// Components
struct Person; 
struct Name(String); 

// Systems
fn hello_world() {
    println!("hello world!");
}

fn add_people(mut commands: Commands) {
    commands
        .spawn((Person, Name("Elaina Proctor".to_string())))
        .spawn((Person, Name("Renzo Hume".to_string())))
        .spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(_person: &Person, name: &Name) {
    println!("hello {}!", name.0);
}
// Entities

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_people.system())
        .add_system(hello_world.system())
        .add_system(greet_people.system());
    }
}

fn main() {
    App::build()
        // Note: .add_plugins(DefaultPlugins) is equivalent to:
        // .add_plugin(CorePlugin::default())
        // .add_plugin(InputPlugin::default())
        // .add_plugin(WindowPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
