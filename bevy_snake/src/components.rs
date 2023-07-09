use bevy::prelude::*;

#[derive(Component)]
pub struct Snake {
    pub direction: Direction,
    pub moved: bool,
    //TODO implement tail with a dequeue
    pub tail: Vec<Entity>,
}

#[derive(Component, Clone)]
pub struct SnakeTail;

#[derive(Component, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
