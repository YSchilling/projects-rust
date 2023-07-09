use bevy::prelude::*;

#[derive(Resource)]
pub struct GameConfig {
    pub block_size: f32,
    pub snake_color: Color,
    pub window_size: (f32, f32),
}

#[derive(Resource)]
pub struct GameState {
    pub move_snake_timer: Timer,
    pub is_running: bool,
}
