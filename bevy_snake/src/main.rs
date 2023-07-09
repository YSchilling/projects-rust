mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use bevy::utils::Duration;
use bevy::window::WindowResolution;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use resources::*;
use systems::*;

fn main() {
    let game_config = GameConfig {
        block_size: 16.,
        snake_color: Color::rgb_u8(51, 204, 51),
        window_size: (512., 512.),
    };

    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(game_config.window_size.0, game_config.window_size.1),
            resizable: false,
            ..default()
        }),
        ..default()
    };

    let game_state = GameState {
        move_snake_timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        is_running: true,
    };

    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_plugin(WorldInspectorPlugin::default())
        .insert_resource(game_config)
        .insert_resource(game_state)
        .add_startup_system(setup)
        .add_systems((turn_snake, tick_timer, move_snake).chain())
        .run();
}
