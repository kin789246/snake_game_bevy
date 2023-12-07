use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct HighScore(pub u16);

#[derive(Default, Resource)]
pub struct SnakeSegments(pub Vec<Entity>);

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone)]
pub struct GameAssets {
    pub cjk_font: Handle<Font>,
    pub head_up: Handle<Image>,
    pub head_down: Handle<Image>,
    pub head_left: Handle<Image>,
    pub head_right: Handle<Image>,
    pub body_bottomleft: Handle<Image>,
    pub body_bottomright: Handle<Image>,
    pub body_horizontal: Handle<Image>,
    pub body_topleft: Handle<Image>,
    pub body_topright: Handle<Image>,
    pub body_vertical: Handle<Image>,
    pub tail_down: Handle<Image>,
    pub tail_left: Handle<Image>,
    pub tail_right: Handle<Image>,
    pub tail_up: Handle<Image>,
    pub apple: Handle<Image>,
}

impl GameAssets {
    pub fn load_assets(
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
