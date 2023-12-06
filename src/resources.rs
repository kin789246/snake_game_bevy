use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct HighScore(pub u16);

#[derive(Default, Resource)]
pub struct SnakeSegments(pub Vec<Entity>);
