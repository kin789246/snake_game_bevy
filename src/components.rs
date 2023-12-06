use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct SnakeHead {
    pub direction: IVec2
}

#[derive(Component)]
pub struct SnakeSegment; 

// #[derive(Component)]
// pub struct SnakeTail;

#[derive(Component)]
pub struct Fruit; 

#[derive(Component)]
pub struct Score(pub u16); 

#[derive(Debug, Component, Default, Copy, Clone, PartialEq)]
pub struct Position(pub IVec2);
