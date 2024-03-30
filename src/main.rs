use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, people_with_jobs)
        .add_systems(Update, people_without_job)
        .add_systems(Update, people_ready_for_hire)
        .run()
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Alex 1".to_string(),
        },
        Employed { job: Job::Doctor },
    ));
    commands.spawn((
        Person {
            name: "Alex 2".to_string(),
        },
        Employed { job: Job::Lawyer },
    ));
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

pub fn people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for person in person_query.iter() {
        println!("{} has a job.", person.name)
    }
}

pub fn people_without_job(person_query: Query<&Person, Without<Employed>>) {
    for person in person_query.iter() {
        println!("{} does not have a job.", person.name)
    }
}

pub fn people_ready_for_hire(person_query: Query<&Person, Without<Employed>>) {
    for person in person_query.iter() {
        println!("{} are ready for hire.", person.name)
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
