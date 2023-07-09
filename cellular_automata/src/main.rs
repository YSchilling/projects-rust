use bevy::{prelude::*, window::WindowResolution};

// For FPS calculation
const TIME_STEP: f32 = 1.0 / 60.0;
const BACKGROUND_COLOR: Color = Color::rgb(0., 0., 0.);
const WINDOW_WIDTH: f32 = 256.;
const WINDOW_HEIGHT: f32 = 256.;
const CELL_SIZE: Vec2 = Vec2::new(8., 8.);

// Resources
#[derive(Resource)]
struct Cells {
    vector: Vec<Vec<Cell>>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, mut cells: ResMut<Cells>) {
    commands.spawn(Camera2dBundle::default());

    let horizontal_cell_count = (WINDOW_WIDTH / CELL_SIZE[0]) as usize;
    let vertical_cell_count = (WINDOW_HEIGHT / CELL_SIZE[1]) as usize;

    for i in 0..horizontal_cell_count {
        for j in 0..vertical_cell_count {
            cells.vector[i][j] = Cell {
                state: false,
                pos: (i, j),
            };
            commands.spawn((cells.vector[i][j], SpatialBundle { ..default() }));
        }
    }
}

// Components

#[derive(Component)]
struct Cell {
    state: bool,
    pos: (usize, usize),
}
