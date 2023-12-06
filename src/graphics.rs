use bevy::prelude::*;

use crate::{
    GameAssets,
    components::{Position, SnakeSegment, Fruit, SnakeHead}, 
    resources::SnakeSegments,
    prelude::*,
};

pub fn snake_transform(
    mut transforms: Query<(&Position, &mut Transform)>,
    heads: Query<&SnakeHead>,
    mut handles: Query<&mut Handle<Image>, With<SnakeSegment>>,
    segments: ResMut<SnakeSegments>,
    game_assets: Res<GameAssets>,
) {
    for (pos, mut transform) in transforms.iter_mut() {
        transform.translation = to_game_xyz(pos.0.x, pos.0.y, 1);
    }
    // head direction
    let head = heads.single();
    if let Ok(mut handle) = handles.get_mut(*segments.0.first().unwrap()) {
        match (head.direction.x as i32, head.direction.y as i32) {
            (1, 0) => *handle = game_assets.head_right.clone(),
            (0, 1) => *handle = game_assets.head_up.clone(),
            (-1, 0) => *handle = game_assets.head_left.clone(),
            (0, -1) => *handle = game_assets.head_down.clone(),
            _ => ()
        }
    }
    // tail direction
    let tail_id = segments.0.last().unwrap();
    let prev_tail_id = segments.0.iter().nth(segments.0.len()-2).unwrap();
    if let Ok((pos_t, _)) = transforms.get(*tail_id) {
        if let Ok((pos_p, _)) = transforms.get(*prev_tail_id) {
            let (dx, dy) = (pos_p.0 - pos_t.0).into();
            if let Ok(mut handle_t) = handles.get_mut(*tail_id) {
                match (dx, dy) {
                    (-1, 0) => *handle_t = game_assets.tail_right.clone(),
                    (0, -1) => *handle_t = game_assets.tail_up.clone(),
                    (1, 0) => *handle_t = game_assets.tail_left.clone(),
                    (0, 1) => *handle_t = game_assets.tail_down.clone(),
                    _ => ()
                }
            }
        }
    }
    // body direction
    for i in 1..(segments.0.len()-1) {
        let prev = transforms.get(segments.0[i-1]).unwrap(); 
        let curr = transforms.get(segments.0[i]).unwrap();
        let next = transforms.get(segments.0[i+1]).unwrap();
        let mut curr_handle = handles.get_mut(segments.0[i]).unwrap();
        let (pc_x, pc_y) = (prev.0.0 - curr.0.0).into();
        let (cn_x, cn_y) = (curr.0.0 - next.0.0).into();
        match (pc_x, pc_y, cn_x, cn_y) {
            (-1, 0, 0, 1) => *curr_handle = game_assets.body_bottomleft.clone(),
            (0, -1, 1, 0) => *curr_handle = game_assets.body_bottomleft.clone(),
            (1, 0, 0, 1) => *curr_handle = game_assets.body_bottomright.clone(),
            (0, -1, -1, 0) => *curr_handle = game_assets.body_bottomright.clone(),
            (-1, 0,-1, 0) => *curr_handle = game_assets.body_horizontal.clone(),
            (1, 0, 1, 0) => *curr_handle = game_assets.body_horizontal.clone(),
            (0, 1, 1, 0) => *curr_handle = game_assets.body_topleft.clone(),
            (-1, 0, 0, -1) => *curr_handle = game_assets.body_topleft.clone(),
            (0, 1, -1, 0) => *curr_handle = game_assets.body_topright.clone(),
            (1, 0, 0, -1) => *curr_handle = game_assets.body_topright.clone(),
            (0, 1, 0, 1) => *curr_handle= game_assets.body_vertical.clone(),
            (0, -1, 0, -1) => *curr_handle = game_assets.body_vertical.clone(),
            _ => ()
        }
    }
}

pub fn fruit_transform(
    mut transforms: Query<(&Position, &mut Transform), With<Fruit>>,
) {
    for (fruit_pos, mut fruit_trans) in transforms.iter_mut() {
        fruit_trans.translation = to_game_xyz(
            fruit_pos.0.x, 
            fruit_pos.0.y,
            1
        );
    }
}

pub fn to_game_xyz(x: i32, y: i32, z: i32) -> Vec3 {
    Vec3::new(
        x as f32 * SNAKE_WIDTH,
        y as f32 * SNAKE_WIDTH - BOARD_OFFSET_Y/2.,
        z as f32
    )
}
