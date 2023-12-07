use bevy::prelude::*;
use rand::prelude::*;
use crate::{
    GameAssets,
    GameState,
    despawn_screen,
    resources::SnakeSegments, 
    components::*, 
    events::*,
    prelude::*,
    graphics::to_game_xyz
};

pub fn new_game(
    commands: Commands,
    query: Query<Entity, With<OnGameScreen>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    info!("new game");
    despawn_screen(query, commands);
    game_state.set(GameState::Play);
}

pub fn init_snake(
    mut commands: Commands,
    mut segments: ResMut<SnakeSegments>,
    game_assets: Res<GameAssets>,
    // meshes: ResMut<Assets<Mesh>>,
    // materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_snake(&mut commands, &mut segments, &game_assets/*, meshes, materials*/);
}

pub fn init_wall(
    mut commands: Commands,
) {
    draw_wall(&mut commands);
}

pub fn init_fruit(
    mut commands: Commands,
    positions: Query<&Position, With<SnakeSegment>>,
    game_assets: Res<GameAssets>,
) {
    spawn_fruit(&mut commands, positions, &game_assets);
}

pub fn spawn_fruit(
    commands: &mut Commands,
    positions: Query<&Position, With<SnakeSegment>>,
    game_assets: &Res<GameAssets>,
) {
    let translation = generate_position(positions);
    commands.spawn((
        SpriteBundle {
            texture: game_assets.apple.clone(),
            transform: Transform::from_translation(
                to_game_xyz(translation.x, translation.y, 1)),
            ..default()
        },
        OnGameScreen,
        Fruit,
        Position(translation),
    ));
}

fn generate_position(
    positions: Query<&Position, With<SnakeSegment>>,
) -> IVec2 {
    let mut result;
    let board_rows = BOARD_ROWS as i32 / 2;
    let board_cols = BOARD_COLS as i32 / 2;
    let mut rng = thread_rng();
    result = IVec2::new(
        rng.gen_range(-board_cols..=board_cols),
        rng.gen_range(-board_rows..=board_rows)
    );

    if positions.is_empty() {
        let pos: Vec<(i32, i32)> = vec![(-1, 0), (0, 0), (1, 0)];
        if pos.contains(&result.into()) {
            result = generate_position(positions);
        }
    }
    else if positions.iter().any(|p| p.0 == result) {
        result = generate_position(positions);
    }

    result
}

pub fn draw_wall(
    commands: &mut Commands,
) {
    // top wall
    commands.spawn((SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(Vec2::new(WINDOW_WIDTH, WALL_WIDTH)),
                ..default()
            },
            transform: Transform::from_translation(
                Vec3::new(0., WINDOW_HEIGHT/2. - WALL_WIDTH/2. - BOARD_OFFSET_Y, 1.)),
            ..default()
        },
        OnGameScreen,
    ));
    // bottom wall
    commands.spawn((SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(Vec2::new(WINDOW_WIDTH, WALL_WIDTH)),
                ..default()
            },
            transform: Transform::from_translation(
                Vec3::new(0., -WINDOW_HEIGHT/2. + WALL_WIDTH/2., 1.)),
            ..default()
        },
        OnGameScreen,
    ));
    // left wall
    commands.spawn((SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(Vec2::new(WALL_WIDTH, BOARD_HEIGHT)),
                ..default()
            },
            transform: Transform::from_translation(
                Vec3::new(
                    (-WINDOW_WIDTH + WALL_WIDTH)/2., 
                    -BOARD_OFFSET_Y/2.,
                    1.
                )
            ),
            ..default()
        },
        OnGameScreen,
    ));
    // right wall
    commands.spawn((SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(Vec2::new(WALL_WIDTH, BOARD_HEIGHT)),
                ..default()
            },
            transform: Transform::from_translation(
                Vec3::new(
                    (WINDOW_WIDTH - WALL_WIDTH)/2., 
                    -BOARD_OFFSET_Y/2.,
                    1.
                )
            ),
            ..default()
        },
        OnGameScreen,
    ));

    // draw board grid
    let b_rows = BOARD_ROWS as i32 / 2;
    let b_cols = BOARD_COLS as i32 / 2;
    let line_color = Color::DARK_GRAY;
    for y in -b_rows..b_rows+2 {
        commands.spawn((SpriteBundle {
            sprite: Sprite {
                color: line_color,
                custom_size: Some(Vec2::new(BOARD_WIDTH, 2.)),
                ..default()
            },
            transform: Transform::from_translation(
                Vec3::new(
                    0., 
                    -BOARD_OFFSET_Y/2. + SNAKE_WIDTH * (-0.5 + y as f32),
                    0.
                )),
                ..default()
            },
            OnGameScreen,
        ));
        for x in -b_cols..b_cols+2 {
            commands.spawn((SpriteBundle {
                sprite: Sprite {
                    color: line_color,
                    custom_size: Some(Vec2::new(2., BOARD_HEIGHT)),
                    ..default()
                },
                transform: Transform::from_translation(
                    Vec3::new(
                        SNAKE_WIDTH * (-0.5 + x as f32),
                        -BOARD_OFFSET_Y/2.,
                        0.
                    )),
                    ..default()
                },
                OnGameScreen,
            ));
        }
    }
}

fn spawn_snake(
    commands: &mut Commands,
    segments: &mut ResMut<SnakeSegments>,
    game_assets: &Res<GameAssets>,
) {
    segments.0 = vec![
        commands.spawn((
            SpriteBundle {
                texture: game_assets.head_right.clone(),
                transform: Transform::from_translation(to_game_xyz(1, 0, 1)),
                ..default()
            },
            OnGameScreen,
            SnakeHead::default(),
            SnakeSegment,
            Position(IVec2::new(1, 0)),
        )).id(),
        commands.spawn((
            SpriteBundle {
                texture: game_assets.body_horizontal.clone(),
                transform: Transform::from_translation(to_game_xyz(0, 0, 1)),
                ..default()
            },
            OnGameScreen,
            SnakeSegment,
            Position(IVec2::new(0, 0)),
        ))
        .id(),
        commands.spawn((
            SpriteBundle {
                texture: game_assets.tail_left.clone(),
                transform: Transform::from_translation(to_game_xyz(-1, 0, 1)),
                ..default()
            },
            OnGameScreen,
            SnakeSegment,
            Position(IVec2::new(-1, 0)),
        ))
        .id(),
    ];
}

pub fn move_snake(
    heads: Query<(Entity, &SnakeHead)>,
    mut bodies: Query<&mut Position>,
    segments: ResMut<SnakeSegments>,
    mut gamover_evw: EventWriter<GameOverEvent>,
) {
    let mut positions: Vec<Position> = Vec::new();
    segments
        .0
        .iter()
        .for_each(|e| {
            if let Ok(pos) = bodies.get_mut(*e) {
                positions.push(*pos);
            }
        });
 
    let (head_id, head) = heads.single();
    if let Ok(mut head_pos) = bodies.get_mut(head_id) {
        let (nx, ny) = (
            head_pos.0.x + head.direction.x,
            head_pos.0.y + head.direction.y
        );
        // collidle with self
        if positions.iter().skip(1)
            .any(|p| p.0 == IVec2::new(nx, ny)) {
                gamover_evw.send(GameOverEvent);
                return;
        }
        
        // collide with wall
        let board_rows = BOARD_ROWS as i32 / 2;
        let board_cols = BOARD_COLS as i32 / 2;
        if nx < -board_cols || board_cols < nx ||
            ny < -board_rows || board_rows < ny {
                gamover_evw.send(GameOverEvent);
                return;
        }
        head_pos.0.x += head.direction.x;
        head_pos.0.y += head.direction.y;

        // set position of segment to previous position and direction of segment
        if head.direction != IVec2::ZERO {
            positions
                .iter()
                .zip(segments.0.iter().skip(1))
                .for_each(|(prev, curr_seg)| {
                    if let Ok(mut pos) = bodies.get_mut(*curr_seg) {
                        pos.0 = prev.0;
                    }
                });
        }
    }
}

pub fn eat(
    mut commands: Commands,
    heads: Query<&Position, With<SnakeHead>>,
    fruits: Query<(Entity, &Position), With<Fruit>>,
    mut score_changed_evw: EventWriter<ScoreChangedEvent>,
    mut spawn_fruit_evw: EventWriter<SpawnFruitEvent>,
    mut growth_evw: EventWriter<GrowthEvent>
) {
    let head_pos = heads.single();
    for (fruit_id, fruit_pos) in fruits.iter() {
        if fruit_pos.0 == head_pos.0 {
            // add scores
            score_changed_evw.send(ScoreChangedEvent);
            // despawn entity
            commands.entity(fruit_id).despawn_recursive();
            // re-spawn fruit
            spawn_fruit_evw.send(SpawnFruitEvent);
            // growth snake
            growth_evw.send(GrowthEvent);
            break;
        }
    }
}

pub fn random_fruit(
    mut commands: Commands,
    positions: Query<&Position, With<SnakeSegment>>,
    game_assets: Res<GameAssets>,
) {
    spawn_fruit(&mut commands, positions, &game_assets);
}

pub fn growth(
    mut commands: Commands,
    snake_q: Query<&Position>,
    mut handle_q: Query<&mut Handle<Image>>,
    mut segments: ResMut<SnakeSegments>,
    game_assets: Res<GameAssets>,
) {
    // change tail to body
    let tail_id = segments.0.last().unwrap();
    let prev_tail_id = segments.0.iter().nth(segments.0.len()-2).unwrap();
    let prev_tail = snake_q.get(*prev_tail_id).unwrap();
    let tail = snake_q.get(*tail_id).unwrap(); 
    let (dx, dy) = (prev_tail.0 - tail.0).into();
            // new tail image
            let texture = match (dx, dy) {
                (-1, 0) => game_assets.tail_right.clone(),
                (0, -1) => game_assets.tail_up.clone(),
                (1, 0) => game_assets.tail_left.clone(),
                (0, 1) => game_assets.tail_down.clone(),
                _ => game_assets.tail_up.clone()
            };
            // tail becomes body and change its image
            let mut tail_handle = handle_q.get_mut(*tail_id).unwrap();
            match (dx, dy) {
                (-1, 0) | (1, 0) => *tail_handle = game_assets.body_horizontal.clone(),
                (0, -1) | (0, 1) => *tail_handle = game_assets.body_vertical.clone(),
                _ => ()
            }
            let (nx, ny, nz) = match (dx, dy) {
                (-1, 0) => (tail.0.x + 1, tail.0.y, 1),
                (0, -1) => (tail.0.x, tail.0.y + 1, 1),
                (1, 0) => (tail.0.x - 1, tail.0.y, 1),
                (0, 1) => (tail.0.x, tail.0.y - 1, 1),
                // set position outside the board
                _ => (BOARD_COLS as i32, BOARD_ROWS as i32, 0)
            };

            segments.0.push(
                commands.spawn((
                        SpriteBundle {
                            texture,
                            transform: Transform::from_translation(to_game_xyz(nx, ny, nz)),
                            ..default()
                        },
                        OnGameScreen,
                        SnakeSegment,
                        Position(IVec2::new(nx, ny)),
                    ))
                    .id(),
            );
}

pub fn score_change(
    mut score_q: Query<(&mut Text, &mut Score), With<Score>>
) {
    if let Ok((mut text, mut score)) = score_q.get_single_mut() {
        score.0 += 1;
        text.sections[0].value = "分數: ".to_string() + &score.0.to_string();
        // println!("text: {:?} score: {:?}", text.sections[0].value, score.0);
    }
}

pub fn game_over(
    mut snake_query: Query<&mut SnakeHead>,
    mut game_state: ResMut<NextState<GameState>>
) {
    println!("Game Over");
    if let Ok(mut head) = snake_query.get_single_mut() {
        head.direction = IVec2::ZERO;
    }

    game_state.set(GameState::Over);
}

pub fn pause_game(
    mut snake_query: Query<&mut SnakeHead>,
    mut menu_q: Query<(&mut Visibility, &OnPlayMenuScreen)>,
) {
    // println!("game is paused");
    
    // freeze all segments
    if let Ok(mut head) = snake_query.get_single_mut() {
        head.direction = IVec2::ZERO;
    }

    if let Ok((mut menu_visibility, _)) = menu_q.get_single_mut() {
        // println!("hide play menu");
        *menu_visibility = Visibility::Hidden;
    }
}

pub fn unpause_game(
    mut menu_q: Query<(&mut Visibility, &OnPlayMenuScreen)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    // println!("game is unpaused");
    game_state.set(GameState::Play);

    if let Ok((mut menu_visibility, _)) = menu_q.get_single_mut() {
        // println!("un-hide play menu");
        *menu_visibility = Visibility::Visible;
    }
}
