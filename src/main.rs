use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, print_names)
        .run()
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Person {
        name: "Alex 1".to_string(),
    });
    commands.spawn(Person {
        name: "Alex 2".to_string(),
    });
    commands.spawn(Person {
        name: "Alex 3".to_string(),
    });
    commands.spawn(Person {
        name: "Alex 4".to_string(),
    });
    commands.spawn(Person {
        name: "Alex 5".to_string(),
    });
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Doctor,
    Firefighter,
    Lawyer,
}
