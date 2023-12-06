use bevy::ecs::event::Event;

#[derive(Event)]
pub struct GameOverEvent;

#[derive(Event)]
pub struct GrowthEvent;

#[derive(Event)]
pub struct SpawnFruitEvent;

#[derive(Event)]
pub struct CollisionEvent;

#[derive(Event)]
pub struct ScoreChangedEvent;
