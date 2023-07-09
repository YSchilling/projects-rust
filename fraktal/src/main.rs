use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}

#[derive(Resource)]
struct AutomataRules {}

#[derive(Component)]
struct Cell {
    state: bool,
}
