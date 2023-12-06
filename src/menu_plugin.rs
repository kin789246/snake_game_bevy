use bevy::prelude::*;
use bevy::app::AppExit;
use crate::{
    prelude::*,
    GameState,
    MenuState,
    GameAssets,
    despawn_screen,
    components::Score,
    game_plugin::OnGameScreen
};

#[derive(Component)]
enum MenuButtonAction {
    New,
    Pause,
    Resume,
    Quit,
}

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

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<MenuState>()
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnExit(MenuState::Main), 
                despawn_screen::<OnMainMenuScreen>)
            .add_systems(OnEnter(MenuState::Pause), pause_menu_setup)
            .add_systems(OnExit(MenuState::Pause), 
                despawn_screen::<OnPauseMenuScreen>)
            //.add_systems(OnEnter(MenuState::Play), play_menu_setup)
            //.add_systems(OnExit(MenuState::Play), hide_play_menu)
            .add_systems(Update, (
                menu_action, 
                button_system,
                menu_keys.run_if(in_state(GameState::Menu))
            ));
    }
}

fn main_menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_assets: Res<GameAssets>,
) {
    let button_style = Style {
        width: Val::Px(250.),
        height: Val::Px(65.),
        margin: UiRect::all(Val::Px(20.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_style = Style {
        width: Val::Px(30.),
        position_type: PositionType::Absolute,
        left: Val::Px(10.),
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        font: game_assets.cjk_font.clone(),
    };

    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            /*"SNAKE GAME",*/ "貪食蛇",
                            TextStyle {
                                font_size: 80.0,
                                color: TEXT_COLOR,
                                font: game_assets.cjk_font.clone(),
                            },
                        )
                        .with_style(
                            Style { 
                                margin: UiRect::all(Val::Px(50.)),
                                ..default()
                            }
                        )
                    );
                    // Display buttons
                    //  - new game
                    //  - quit
                    parent.spawn((
                        ButtonBundle {
                            style: button_style.clone(),
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        },
                        MenuButtonAction::New,
                        MenuItems,
                        ToSelectOption
                    ))
                    .with_children(|parent| {
                        let icon = asset_server.load(
                            "textures/game_icons/right.png");
                        parent.spawn(
                            ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            }
                        );
                        parent.spawn(
                            TextBundle::from_section(
                            /*"NEW GAME",*/ "新遊戲",
                                button_text_style.clone()
                            )
                        );
                    });
                    parent.spawn((
                        ButtonBundle {
                            style: button_style.clone(),
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        },
                        MenuButtonAction::Quit,
                        MenuItems
                    ))
                    .with_children(|parent| {
                        let icon = asset_server.load("textures/game_icons/exitRight.png");
                        parent.spawn(ImageBundle {
                            style: button_icon_style,
                            image: UiImage::new(icon),
                            ..default()
                        });
                        parent.spawn(TextBundle::from_section("離開", button_text_style));
                    });
                });
        });
}

fn pause_menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_assets: Res<GameAssets>,
    curr_game_state: Res<State<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>
) {
    if curr_game_state.get() == &GameState::Over {
        menu_state.set(MenuState::Main);
        return;
    }

    let button_style = Style {
        width: Val::Px(250.),
        height: Val::Px(65.),
        margin: UiRect::all(Val::Px(20.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_style = Style {
        width: Val::Px(30.),
        position_type: PositionType::Absolute,
        left: Val::Px(10.),
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        font: game_assets.cjk_font.clone(),
    };

    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
            },
            OnPauseMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            "貪食蛇",
                            TextStyle {
                                font_size: 80.0,
                                color: TEXT_COLOR,
                                font: game_assets.cjk_font.clone(),
                            },
                        )
                        .with_style(
                            Style { 
                                margin: UiRect::all(Val::Px(50.)),
                                ..default()
                            }
                        )
                    );
                    // Display buttons
                    //  - resume
                    //  - new game
                    //  - quit
                    parent.spawn((
                        ButtonBundle {
                            style: button_style.clone(),
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        },
                        MenuButtonAction::Resume,
                        MenuItems,
                        ToSelectOption
                    ))
                    .with_children(|parent| {
                        parent.spawn(
                            TextBundle::from_section(
                                "回到遊戲",
                                button_text_style.clone()
                            )
                        );
                    });
                    parent.spawn((
                        ButtonBundle {
                            style: button_style.clone(),
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        },
                        MenuButtonAction::New,
                        MenuItems
                    ))
                    .with_children(|parent| {
                        let icon = asset_server.load(
                            "textures/game_icons/right.png");
                        parent.spawn(
                            ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            }
                        );
                        parent.spawn(
                            TextBundle::from_section(
                            /*"NEW GAME",*/ "新遊戲",
                                button_text_style.clone()
                            )
                        );
                    });
                    parent.spawn((
                        ButtonBundle {
                            style: button_style.clone(),
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        },
                        MenuButtonAction::Quit,
                        MenuItems
                    ))
                    .with_children(|parent| {
                        let icon = asset_server.load("textures/game_icons/exitRight.png");
                        parent.spawn(ImageBundle {
                            style: button_icon_style,
                            image: UiImage::new(icon),
                            ..default()
                        });
                        parent.spawn(TextBundle::from_section(
                            "離開", button_text_style)
                        );
                    });
                });
        });
}

pub fn play_menu_setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    let button_text_style = TextStyle {
        font_size: 25.0,
        color: TEXT_COLOR,
        font: game_assets.cjk_font.clone(),
    };

    commands
        .spawn((NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Px(BOARD_OFFSET_Y),
                grid_template_columns: vec![
                    GridTrack::percent(30.), 
                    GridTrack::percent(30.),
                    GridTrack::auto()
                ],
                ..default()
            }, 
            ..default()
            },
            OnPlayMenuScreen,
            OnGameScreen,
        )) 
        .with_children(|parent| {
            // pause button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(50.),
                        height: Val::Auto,
                        margin: UiRect::all(Val::Px(5.)),
                        grid_column: GridPlacement::start(1),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                MenuButtonAction::Pause,
            ))
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        "暫停",
                        button_text_style.clone()
                    )
                );
            });

            // score board
            parent.spawn((TextBundle::from_section(
                    "分數: 0".to_string(), 
                    button_text_style.clone()
                ).with_style(Style {
                    grid_column: GridPlacement::start(3),
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::End,
                    margin: UiRect::all(Val::Px(3.)),
                    ..default()
                }),
                Score(0)
            ));
        }); 
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&ToSelectOption>),
        (Changed<Interaction>, With<Button>)
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) => PRESSED_BUTTON.into(),
            (Interaction::None, Some(_)) => HOVERED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>)
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menut_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menut_button_action {
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
                MenuButtonAction::New => {
                    #[cfg(feature = "debug")]
                    info!("menu act new");
                    game_state.set(GameState::New);
                    menu_state.set(MenuState::Play);
                },
                MenuButtonAction::Resume => {
                    #[cfg(feature = "debug")]
                    info!("menu act resume");
                    game_state.set(GameState::Resume);
                    menu_state.set(MenuState::Play);
                },
                MenuButtonAction::Pause => {
                    #[cfg(feature = "debug")]
                    info!("game pause");
                    game_state.set(GameState::Menu);
                    menu_state.set(MenuState::Pause);
                }
            }
        }
    }
}

fn menu_keys(
    mut commands: Commands,
    mut interactions: Query<
        (Entity, &Transform, &mut BackgroundColor, Option<&ToSelectOption>), 
        With<MenuItems>
    >,
    kb_input: Res<Input<KeyCode>>,
    menu_actions: Query<
        (&MenuButtonAction, Option<&ToSelectOption>),
        With<Button>
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
    curr_menu_state: Res<State<MenuState>>
) {
    let mut interactions_vec = interactions.iter_mut().collect::<Vec<_>>();
    interactions_vec.sort_by_key(|c| c.1.translation.y as i32);
    if kb_input.any_just_pressed([KeyCode::Down, KeyCode::S, KeyCode::J]) {
        let mut focused = interactions_vec.iter_mut();
        while let Some((entity, _trans, bgcolor, maybe_select)) = 
            focused.next() {
            if maybe_select.is_some() {
                commands.entity(*entity).remove::<ToSelectOption>();
                **bgcolor = NORMAL_BUTTON.into();
                break;
            }
        }
        if let Some((entity, _trans, bgcolor, _maybe_select)) =
            focused.next() {
            commands.entity(*entity).insert(ToSelectOption);
            **bgcolor = HOVERED_BUTTON.into();
        }
        else {
            if let Some(first) = interactions_vec.iter_mut().next() {
                commands.entity((*first).0).insert(ToSelectOption);
                *(*first).2 = HOVERED_BUTTON.into();
            }
        }
    }

    if kb_input.any_just_pressed([KeyCode::Up, KeyCode::W, KeyCode::K]) {
        let mut focused = interactions_vec.iter_mut().rev();
        while let Some((entity, _trans, bgcolor, maybe_select)) = 
            focused.next() {
            if maybe_select.is_some() {
                commands.entity(*entity).remove::<ToSelectOption>();
                **bgcolor = NORMAL_BUTTON.into();
                break;
            }
        }
        if let Some((entity, _trans, bgcolor, _maybe_select)) =
            focused.next() {
            commands.entity(*entity).insert(ToSelectOption);
            **bgcolor = HOVERED_BUTTON.into();
        }
        else {
            if let Some(first) = interactions_vec.iter_mut().rev().next() {
                commands.entity((*first).0).insert(ToSelectOption);
                *(*first).2 = HOVERED_BUTTON.into();
            }
        }
    }

    if kb_input.any_just_pressed([KeyCode::Return, KeyCode::Space]) {
        for (menu_action, maybe_select) in menu_actions.iter() {
            if maybe_select.is_some() {
                match menu_action {
                    MenuButtonAction::Quit => app_exit_events.send(AppExit),
                    MenuButtonAction::New => {
                        #[cfg(feature = "debug")]
                        info!("keyboard new");
                        game_state.set(GameState::New);
                        menu_state.set(MenuState::Play);
                    },
                    MenuButtonAction::Resume => {
                        #[cfg(feature = "debug")]
                        info!("keyboard resume");
                        game_state.set(GameState::Resume);
                        menu_state.set(MenuState::Play);
                    },
                    MenuButtonAction::Pause => {
                        #[cfg(feature = "debug")]
                        info!("keyboard pause");
                        game_state.set(GameState::Menu);
                        menu_state.set(MenuState::Pause);
                    }
                }
            }
        }
    }

    // resume game directly
    if curr_menu_state.get() == &MenuState::Pause {
        if kb_input.just_pressed(KeyCode::Escape) {
            #[cfg(feature = "debug")]
            info!("keyboard resume");
            game_state.set(GameState::Resume);
            menu_state.set(MenuState::Play);
        }
    }
}
