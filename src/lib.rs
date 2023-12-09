use bevy::prelude::*;
//use bevy::window::PresentMode;
use resources::{
    GameSetting, 
    GameAssets
};
use components::MainCamera;
use states::*;

mod game_plugin;
mod menu_plugin;
mod control;
mod input;
mod events;
mod components;
mod resources;
mod graphics;
mod states;

mod prelude {
    use bevy::prelude::*;
    pub const GAME_FPS: u64 = (1000. / 60. * 40. / 3.) as u64;
    pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
    pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
    pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
    pub const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
    pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
    pub const WALL_WIDTH: f32 = 10.;
    pub const WALL_COLOR: Color = Color::LIME_GREEN;
    pub const BOARD_ROWS: u32 = 15;
    pub const BOARD_COLS: u32 = 9;
    pub const BOARD_OFFSET_Y: f32 = 40.0;
}

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
                                    9. * 40. + 2. * 10.,
                                    15. * 40. + 2. * 10. + 40.
                                ).into(), 
                                title: "貪食蛇".to_string(), 
                                fit_canvas_to_parent: true,
                                ..default()
                            }
                    ),
                    ..default()
                })
            )
            .add_state::<GameState>()
            .insert_resource(GameSetting::default())
            .add_systems(Startup, 
                (
                    env_setup, 
                    GameAssets::load_assets, 
                    #[cfg(target_arch = "wasm32")]
                    graphics::setup_ui
                ).chain()
            )
            //.add_systems(Update, graphics::on_size_changed)
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
