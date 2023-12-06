use bevy::prelude::*;
//use bevy::window::PresentMode;
use prelude::*;

mod game_plugin;
mod menu_plugin;
mod control;
mod input;
mod events;
mod components;
mod resources;
mod graphics;

mod prelude {
    use bevy::prelude::*;
    pub const GAME_FPS: u64 = (1000.0 / 60.0 * SNAKE_WIDTH / 3.) as u64;
    pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
    pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
    pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
    pub const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
    pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
    pub const SNAKE_WIDTH: f32 = 40.0;
    pub const WALL_WIDTH: f32 = 10.;
    pub const WALL_COLOR: Color = Color::LIME_GREEN;
    pub const BOARD_ROWS: u32 = 15;
    pub const BOARD_COLS: u32 = 15;
    pub const BOARD_OFFSET_Y: f32 = 40.0;
    pub const BOARD_WIDTH: f32 = BOARD_COLS as f32 * SNAKE_WIDTH;
    pub const BOARD_HEIGHT: f32 = BOARD_ROWS as f32 * SNAKE_WIDTH;
    pub const _WIN_PADDING: f32 = 10.;
    pub const WINDOW_WIDTH: f32 = BOARD_COLS as f32 * SNAKE_WIDTH +
        2.0 * WALL_WIDTH;
    pub const WINDOW_HEIGHT: f32 = BOARD_ROWS as f32 * SNAKE_WIDTH +
        2.0 * WALL_WIDTH + BOARD_OFFSET_Y; 
}

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    New,
    Resume,
    Play,
    Over,
}

// state used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    #[default]
    Main,
    Pause,
    Play,
}

#[derive(Component)]
pub struct MainCamera;

pub struct SnakeGame;

impl SnakeGame {
    pub fn run() {
        let mut app = App::new();
        app
            .add_plugins(DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(
                            Window { 
                                resolution: (
                                    WINDOW_WIDTH, WINDOW_HEIGHT
                                ).into(), 
                                title: "貪食蛇".to_string(), 
                                ..default()
                            }
                    ),
                    ..default()
                })
            )
            .add_state::<GameState>()
            .add_systems(Startup, (env_setup, GameAssets::load_assets))
            //.add_systems(Update, toggle_vsync)
            .add_plugins((menu_plugin::MenuPlugin, game_plugin::GamePlugin));

        // #[cfg(feature = "debug")]
        // app
        //     .add_plugins(LogDiagnosticsPlugin::default())
        //     .add_plugins(FrameTimeDiagnosticsPlugin::default());
            
        app.run();
    }
}

fn env_setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone)]
pub struct GameAssets {
    cjk_font: Handle<Font>,
    head_up: Handle<Image>,
    head_down: Handle<Image>,
    head_left: Handle<Image>,
    head_right: Handle<Image>,
    body_bottomleft: Handle<Image>,
    body_bottomright: Handle<Image>,
    body_horizontal: Handle<Image>,
    body_topleft: Handle<Image>,
    body_topright: Handle<Image>,
    body_vertical: Handle<Image>,
    tail_down: Handle<Image>,
    tail_left: Handle<Image>,
    tail_right: Handle<Image>,
    tail_up: Handle<Image>,
    apple: Handle<Image>,
}

impl GameAssets {
    fn load_assets(
        mut commands: Commands,
        asset_server: Res<AssetServer>
) {
        let cjk_font = asset_server.load("fonts/NotoSansMonoCJKtc-Regular.otf");
        let head_up = asset_server.load("textures/snake/head_up.png");
        let head_down = asset_server.load("textures/snake/head_down.png");
        let head_left = asset_server.load("textures/snake/head_left.png");
        let head_right = asset_server.load("textures/snake/head_right.png");
        let body_bottomleft = asset_server.load("textures/snake/body_bottomleft.png");
        let body_bottomright = asset_server.load("textures/snake/body_bottomright.png");
        let body_horizontal = asset_server.load("textures/snake/body_horizontal.png");
        let body_topleft = asset_server.load("textures/snake/body_topleft.png");
        let body_topright = asset_server.load("textures/snake/body_topright.png");
        let body_vertical = asset_server.load("textures/snake/body_vertical.png");
        let tail_down = asset_server.load("textures/snake/tail_down.png");
        let tail_left = asset_server.load("textures/snake/tail_left.png");
        let tail_right = asset_server.load("textures/snake/tail_right.png");
        let tail_up = asset_server.load("textures/snake/tail_up.png");
        let apple = asset_server.load("textures/snake/apple.png");
        
        commands.insert_resource(
            GameAssets {
                cjk_font,
                head_up, head_down, head_left, head_right,
                body_bottomleft, body_bottomright, body_horizontal,
                body_topleft, body_topright, body_vertical,
                tail_down, tail_left, tail_right, tail_up,
                apple,
            }
        );
    }
}

// Generic system that takes a component as a parameter,
// and will despawn all entities with that component
fn despawn_screen<T: Component>(
    to_despawn: Query<Entity, With<T>>,
    mut commands: Commands
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

/// transfer the world coordinate to viewport coordinate
fn _to_viewport(
    world_position: &Vec3,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    camera.world_to_viewport(camera_transform, *world_position)
}

/// transfer the viewport coordinate to world coordinate
fn _to_world(
    viewport_position: &Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    camera.viewport_to_world_2d(camera_transform, *viewport_position)
}
/*
fn toggle_vsync(
    kb_input: Res<Input<KeyCode>>,
    mut windows: Query<&mut Window>
) {
    if kb_input.just_pressed(KeyCode::V) {
        let mut window = windows.single_mut();

        window.present_mode = 
            if matches!(window.present_mode, PresentMode::AutoVsync) {
                PresentMode::AutoNoVsync
            } else {
                PresentMode::AutoVsync
            };
        info!("PRESENT_MODE: {:?}", window.present_mode);
    }
}
*/
