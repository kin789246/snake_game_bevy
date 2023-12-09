use bevy::{
    prelude::*, 
    winit::WinitWindows,
    window::{PrimaryWindow, WindowResized}
};

use crate::{
    GameAssets,
    components::{Position, SnakeSegment, Fruit, SnakeHead, MainCamera}, 
    resources::{SnakeSegments, GameSetting},
    prelude::*,
};

#[cfg(target_arch = "wasm32")]
use bevy::window::WindowCreated;

pub fn snake_transform(
    mut transforms: Query<(&Position, &mut Transform)>,
    heads: Query<&SnakeHead>,
    mut handles: Query<&mut Handle<Image>, With<SnakeSegment>>,
    segments: ResMut<SnakeSegments>,
    game_assets: Res<GameAssets>,
    game_settings: Res<GameSetting>
) {
    for (pos, mut transform) in transforms.iter_mut() {
        transform.translation = to_game_xyz(pos.0.x, pos.0.y, 1,
            game_settings.snake_width);
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
    game_settings: Res<GameSetting>
) {
    for (fruit_pos, mut fruit_trans) in transforms.iter_mut() {
        fruit_trans.translation = to_game_xyz(
            fruit_pos.0.x, 
            fruit_pos.0.y,
            1,
            game_settings.snake_width
        );
    }
}

pub fn to_game_xyz(x: i32, y: i32, z: i32, snake_width: f32) -> Vec3 {
    Vec3::new(
        x as f32 * snake_width,
        y as f32 * snake_width - BOARD_OFFSET_Y/2.,
        z as f32
    )
}

#[cfg(target_arch = "wasm32")]
pub fn setup_ui(
    mut window_q: Query<(Entity, &mut Window), With<PrimaryWindow>>,
    mut game_settings: ResMut<GameSetting>,
    mut window_created_evr: EventReader<WindowCreated>
    // winit_window: NonSend<WinitWindows>, 
) {
    for _event in window_created_evr.read() {
        info!("window created");
        let (_primary_window, mut window) = window_q.single_mut();
        let wasm_window = web_sys::window().unwrap();
        // info!("\ncanvas width:{:?}, height:{:?}",
        //     wasm_window.inner_width(), wasm_window.inner_height());
        let game_width = (BOARD_COLS as f32 * game_settings.snake_width +
            2. * WALL_WIDTH) as f64;
        let game_height = (BOARD_ROWS as f32 * game_settings.snake_width +
            2. * WALL_WIDTH + BOARD_OFFSET_Y) as f64;
        let width = wasm_window.inner_width().unwrap().as_f64().unwrap();
        let height = wasm_window.inner_height().unwrap().as_f64().unwrap();
        info!("portrait mode and w={:?} gw={:?}", width, game_width);
        info!("h={:?} gh={:?}", height, game_height);
        if width >= height {
            if height * 0.90 < game_height {
                let scale = (height * 0.90 / game_height) as f32;
                game_settings.game_scale = scale;
                game_settings.snake_width *= scale;
            }
        }
        else {
            if width < game_width {
                let scale = (width / game_width) as f32;
                game_settings.game_scale = scale;
                game_settings.snake_width *= scale;
            }
            else if height * 0.85 < game_height {
                let scale = (height * 0.85 / game_height) as f32;
                game_settings.game_scale = scale;
                game_settings.snake_width *= scale;
            }
        }
        let win_width = BOARD_COLS as f32 * game_settings.snake_width +
            2. * WALL_WIDTH;
        let win_height = BOARD_ROWS as f32 * game_settings.snake_width +
            2. * WALL_WIDTH + BOARD_OFFSET_Y;
        (*window).resolution.set(win_width, win_height);
    }
    //info!("\nw={:?}, h={:?}", win_width, win_height);
    // let winit = winit_window.get_window(primary_window).unwrap();
    //let monitor = winit.current_monitor().unwrap();
    // let is = winit.inner_size();
    // let sf = winit.scale_factor();
    // let width = is.width as f64 / sf;
    // let height = is.height as f64 / sf;
    
    // info!("\ninner_size={:?}\nscale_factor={:?}", is, sf);
    // info!("\nwidth={:?}, height={:?}", width, height);
    // info!("\nwindow_resolution={:?}", window.resolution);
    // calculate new settings
}

pub fn _on_size_changed(
    winit_window: NonSend<WinitWindows>, 
    mut window_q: Query<Entity, With<PrimaryWindow>>,
    mut sizechanged_evr: EventReader<WindowResized>,
    camera_q: Query<&Camera, With<MainCamera>>
) {
    if let Ok(camera) = camera_q.get_single() {
        info!("\ncamera={:?}", camera);
    }

    for resized in sizechanged_evr.read() {
        info!("\nresized");
        let primary_window = window_q.single_mut();
        let winit = winit_window.get_window(primary_window).unwrap();
        let is = winit.inner_size();
        let sf = winit.scale_factor();
        let width = is.width as f64 / sf;
        let height = is.height as f64 / sf;
        
        info!("\nwin_resized: width={:?}, height={:?}", resized.width, resized.height);
        info!("\ninner_size={:?}\nscale_factor={:?}", is, sf);
        info!("\nwidth={:?}, height={:?}", width, height);
    }
}
