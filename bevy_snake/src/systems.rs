use crate::components::Direction;
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn setup(mut commands: Commands, game_config: Res<GameConfig>) {
    commands.spawn(Camera2dBundle::default());

    let tail = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: game_config.snake_color,
                    rect: Some(Rect::new(
                        0.,
                        0.,
                        game_config.block_size,
                        game_config.block_size,
                    )),
                    ..default()
                },
                transform: Transform::from_translation(Vec3 {
                    x: 0.,
                    y: -game_config.block_size,
                    z: 0.,
                }),
                ..default()
            },
            SnakeTail,
        ))
        .id();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: game_config.snake_color,
                rect: Some(Rect::new(
                    0.,
                    0.,
                    game_config.block_size,
                    game_config.block_size,
                )),
                ..default()
            },
            ..default()
        },
        Snake {
            direction: Direction::Up,
            moved: false,
            tail: vec![tail],
        },
    ));
}

pub fn tick_timer(time: Res<Time>, mut game_state: ResMut<GameState>) {
    game_state.move_snake_timer.tick(time.delta());
}

pub fn move_snake(
    game_config: Res<GameConfig>,
    game_state: Res<GameState>,
    mut q_snake: Query<(&mut Snake, &mut Transform)>,
    mut q_tail: Query<(Entity, &mut Transform), (With<SnakeTail>, Without<Snake>)>,
) {
    for (mut snake, mut transform_head) in q_snake.iter_mut() {
        if game_state.move_snake_timer.finished() {
            snake.moved = true;

            for (entity, mut transform_tail) in q_tail.iter_mut() {
                if entity == *snake.tail.last().unwrap() {
                    transform_tail.translation.x = transform_head.translation.x;
                    transform_tail.translation.y = transform_head.translation.y;
                    transform_tail.translation.z = transform_head.translation.z;
                    let last = snake.tail.pop().unwrap();
                    snake.tail.insert(0, last);
                }
            }

            match snake.direction {
                Direction::Up => transform_head.translation.y += game_config.block_size,
                Direction::Right => transform_head.translation.x += game_config.block_size,
                Direction::Down => transform_head.translation.y -= game_config.block_size,
                Direction::Left => transform_head.translation.x -= game_config.block_size,
            }
        }
    }
}

pub fn turn_snake(keys: Res<Input<KeyCode>>, mut query: Query<&mut Snake>) {
    for mut snake in query.iter_mut() {
        if keys.just_pressed(KeyCode::Up)
            && snake.direction != Direction::Down
            && snake.moved == true
        {
            snake.direction = Direction::Up;
            snake.moved = false;
        }
        if keys.just_pressed(KeyCode::Right)
            && snake.direction != Direction::Left
            && snake.moved == true
        {
            snake.direction = Direction::Right;
            snake.moved = false;
        }
        if keys.just_pressed(KeyCode::Down)
            && snake.direction != Direction::Up
            && snake.moved == true
        {
            snake.direction = Direction::Down;
            snake.moved = false;
        }
        if keys.just_pressed(KeyCode::Left)
            && snake.direction != Direction::Right
            && snake.moved == true
        {
            snake.direction = Direction::Left;
            snake.moved = false;
        }
    }
}

fn check_hitbox(game_config: GameConfig, new ) {
    for transform in query.iter() {
        if transform.translation.x <
    }
}
