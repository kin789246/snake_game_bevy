use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct SnakeHead {
    pub direction: IVec2
}

#[derive(Component)]
pub struct SnakeSegment; 

#[derive(Component)]
pub struct Fruit; 

#[derive(Component)]
pub struct Score(pub u16); 

#[derive(Debug, Component, Default, Copy, Clone, PartialEq)]
pub struct Position(pub IVec2);

#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub struct OnPauseMenuScreen;

#[derive(Component)]
pub struct SelectedOption;

#[derive(Component)]
pub struct OnPlayMenuScreen;

#[derive(Debug, Component)]
pub struct ToSelectOption;

#[derive(Debug, Component)]
pub struct MenuItems;

#[derive(Component)]
pub struct OnGameScreen;
