use bevy::prelude::*;
use crate::{
    GameState,
    MenuState,
    components::*, 
    resources::SnakeSegments,
};

pub fn keyboard(
    kb_input: Res<Input<KeyCode>>,
    mut heads: Query<&mut SnakeHead>,
    bodies: Query<&Position, With<SnakeSegment>>,
    segments: Res<SnakeSegments>
) {
    let mut head = heads.single_mut();
    // detect if new direction against the body
    let head_id = segments.0.first().unwrap();
    let next_head_id = segments.0.iter().nth(1).unwrap();
    let head_pos = bodies.get(*head_id).unwrap();
    let next_pos = bodies.get(*next_head_id).unwrap();
    let (dx, dy) = (head_pos.0 - next_pos.0).into();

    let mut direction = IVec2::ZERO;
    if kb_input.any_just_pressed([KeyCode::Left, KeyCode::A, KeyCode::H]) {
        if (dx, dy) == (1, 0) {
            return;
        }
        direction = IVec2::new(-1, 0);
    }
    else if kb_input.any_just_pressed([KeyCode::Right, KeyCode::D, KeyCode::L]) {
        if (dx, dy) == (-1, 0) {
            return;
        }
        direction = IVec2::new(1, 0);
    }
    else if kb_input.any_just_pressed([KeyCode::Up, KeyCode::W, KeyCode::K]) {
        if (dx, dy) == (0, -1) {
            return;
        }
        direction = IVec2::new(0, 1);
    }
    else if kb_input.any_just_pressed([KeyCode::Down, KeyCode::S, KeyCode::J]) {
        if (dx, dy) == (0, 1) {
            return;
        }
        direction = IVec2::new(0, -1);
    }

    if direction != IVec2::ZERO {
        head.direction = direction;
    }
}

pub fn pause(
    kb_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    if kb_input.any_just_pressed([KeyCode::Space, KeyCode::Escape]) {
        game_state.set(GameState::Menu);
        menu_state.set(MenuState::Pause);
    }
}
