
use bevy::{
    prelude::*,
};

#[derive(Component)]
struct DieValue(u8);

#[derive(Component)]
struct DiceContainer;

#[derive(Bundle)]
struct Die {
    value: DieValue,
    container: DiceContainer
}

impl Die {
    fn new(container: DiceContainer) -> Die {
        Die {
            value: DieValue(1),
            container
        }
    }
}

fn setup(mut commands: Commands) {
    
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

