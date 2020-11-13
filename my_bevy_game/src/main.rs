use bevy::prelude::*;

/* Mario was here */

// Components
struct Person; 
struct Name(String); 

// Resources (Globals)
struct GreetTimer(Timer);

// Systems
fn add_people(mut commands: Commands) {
    commands
        .spawn((Person, Name("Elaina Proctor".to_string())))
        .spawn((Person, Name("Renzo Hume".to_string())))
        .spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, _person: &Person, name: &Name) {
    // Time is a resource added by add_plugins(DefaultPlugins)
    // Timer is used to track the amount of time passed, Timer is provided by Bevy
    timer.0.tick(time.delta_seconds);

    if timer.0.finished {
        println!("hello {}!", name.0);
    }

}
// Entities


// Plugins
pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
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
