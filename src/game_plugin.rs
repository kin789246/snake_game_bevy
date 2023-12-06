use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use crate::{
    control::*,
    GameState, 
    resources::SnakeSegments,
    events::*,
    input::*,
    graphics::*, 
    prelude::*,
    menu_plugin::play_menu_setup
};

#[derive(Component)]
pub struct OnGameScreen;

#[derive(SystemSet, Hash, PartialEq, Eq, Clone, Debug)]
pub enum Phase {
    Input,
    //Movement,
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::New), 
                (
                    new_game, 
                    play_menu_setup,
                    init_snake,
                    init_fruit, 
                    init_wall
                ).chain()
            )
            .add_systems(OnEnter(GameState::Resume), unpause_game)
            .add_systems(OnEnter(GameState::Menu), pause_game)
            .add_systems(Update, (
                pause
                    .in_set(Phase::Input)
                    .run_if(not(in_state(GameState::Menu))),
                (
                    keyboard.in_set(Phase::Input),
                    (
                        move_snake.after(Phase::Input),
                        eat,
                        snake_transform,
                        fruit_transform,
                    )
                    .chain()
                    .run_if(on_timer(Duration::from_millis(GAME_FPS))),
                    growth.run_if(on_event::<GrowthEvent>()),
                    random_fruit.run_if(on_event::<SpawnFruitEvent>()),
                    game_over.run_if(on_event::<GameOverEvent>()),
                    score_change.run_if(on_event::<ScoreChangedEvent>())
                )
                .run_if(in_state(GameState::Play))
            ))
            .add_event::<GameOverEvent>()
            .add_event::<SpawnFruitEvent>()
            .add_event::<GrowthEvent>()
            .add_event::<CollisionEvent>()
            .add_event::<ScoreChangedEvent>()
            .insert_resource(SnakeSegments::default());
    }
}
