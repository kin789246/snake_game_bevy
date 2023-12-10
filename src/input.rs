use bevy::prelude::*;
use bevy::input::touch::TouchPhase;
use crate::{
    GameState,
    MenuState,
    components::*, 
    resources::{SnakeSegments, TouchPosition},
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
pub fn touch_input(
    touches: Res<Touches>,
    mut heads: Query<&mut SnakeHead>,
    bodies: Query<&Position, With<SnakeSegment>>,
    segments: Res<SnakeSegments>
) {
    for finger in touches.iter_just_released() {
        // info!("just released finger {:?}", finger.id());
        let (dx, dy) = finger.distance().into();
        // info!("dx{:?} dy{:?}", dx, dy);
        if (dx, dy) == (0., 0.) {
            // not swipe
            return;
        }

        if dx.abs() > dy.abs() {
            if dx > 0. {
                // swipe right
                handle_swipe(IVec2::new(1, 0), &mut heads, &bodies, &segments);
            }
            else {
                // swipe left
                handle_swipe(IVec2::new(-1, 0), &mut heads, &bodies, &segments);
            }
        }
        else {
            if dy < 0. {
                // swipe down
                handle_swipe(IVec2::new(0, 1), &mut heads, &bodies, &segments);
            }
            else {
                // swipe up
                handle_swipe(IVec2::new(0, -1), &mut heads, &bodies, &segments);
            }
        }
    }
}

pub fn _touch_events(
    mut touch_evr: EventReader<TouchInput>,
    mut touch_start: ResMut<TouchPosition>,
    mut heads: Query<&mut SnakeHead>,
    bodies: Query<&Position, With<SnakeSegment>>,
    segments: Res<SnakeSegments>
) {
    for touch_input in touch_evr.read() {
        match touch_input.phase {
            TouchPhase::Started => {
                touch_start.0 = touch_input.position;
                //info!("touch started");
            },
            TouchPhase::Ended => {
                let (dx, dy) = (touch_input.position - touch_start.0).into();
                info!("dx{:?} dy{:?}", dx, dy);
                if dx.abs() > dy.abs() {
                    if dx > 0. {
                        // swipe right
                        handle_swipe(IVec2::new(1, 0), &mut heads, &bodies, &segments);
                    }
                    else {
                        // swipe left
                        handle_swipe(IVec2::new(-1, 0), &mut heads, &bodies, &segments);
                    }
                }
                else {
                    if dy < 0. {
                        // swipe down
                        handle_swipe(IVec2::new(0, 1), &mut heads, &bodies, &segments);
                    }
                    else {
                        // swipe up
                        handle_swipe(IVec2::new(0, -1), &mut heads, &bodies, &segments);
                    }
                }
                touch_start.0 = Vec2::ZERO;
                //info!("touch ended => {:?}", result);
            },
            TouchPhase::Canceled => {
                touch_start.0 = Vec2::ZERO;
            },
            TouchPhase::Moved => ()
        }
    }
}


pub fn handle_swipe(
    swipe_to: IVec2,
    heads: &mut Query<&mut SnakeHead>,
    bodies: &Query<&Position, With<SnakeSegment>>,
    segments: &Res<SnakeSegments>
) {
    let mut head = heads.single_mut();
    // detect if new direction against the body
    let head_id = segments.0.first().unwrap();
    let next_head_id = segments.0.iter().nth(1).unwrap();
    let head_pos = bodies.get(*head_id).unwrap();
    let next_pos = bodies.get(*next_head_id).unwrap();
    let (dx, dy) = (head_pos.0 - next_pos.0).into();

    if swipe_to == IVec2::new(-1, 0) && (dx, dy) == (1, 0) {
        // swipe left, head move to right
        return;
    }
    else if swipe_to == IVec2::new(1, 0) && (dx, dy) == (-1, 0) {
        // swipe right, head move to left
        return;
    }
    else if swipe_to == IVec2::new(0, -1) && (dx, dy) == (0, 1) {
        // swipe up, head move to down
        return;
    }
    else if swipe_to == IVec2::new(0, 1) && (dx, dy) == (0, -1) {
        // swipe down, head move to up
        return;
    }

    head.direction = swipe_to;
}
