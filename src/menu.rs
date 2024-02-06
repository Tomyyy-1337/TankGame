use bevy::prelude::*;
use bevy_framepace::FramepaceSettings;

use crate::asset_loader::FontAssets;
use crate::schedule::ScheduleSet;

#[derive(Component)]
pub struct FortsetzenButton;

#[derive(Component)]
pub struct FPSButton;

#[derive(Component)]
pub struct QuitButton;

/// Marker component for menu items.
#[derive(Component)]
pub struct MenuItem;


/// Plugin for the menu system.
pub struct MenuPlugin;


/// The different states of the menu. The menu can be open or closed.
#[derive(States, Debug, Hash, Clone, Eq, PartialEq)]
pub enum MenuState {
    Closed,
    Open,
}

/// Default implementation for the `MenuState` enum. The default state is `MenuState::Closed`.
impl Default for MenuState {
    fn default() -> Self {
        MenuState::Closed
    }
}

/// Implementation of the `Plugin` trait for the `MenuPlugin` struct.
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
            .add_systems(Update, (
                toggle_menu,
                clear_menu,
            ).in_set(ScheduleSet::CheckMenu))
            .add_systems(Update, (
                toggle_framerate_lock,
                fortsetzen_button,
                quit_game_button,
                button_hower,
            ).in_set(ScheduleSet::PauseMenu));
    }
        
}

fn clear_menu(
    mut commands: Commands,
    simulation_state: Res<State<MenuState>>,
    mut query: Query<(Entity, &MenuItem)>,
) {
    match simulation_state.get() {
        MenuState::Closed => {
            for (entity, _) in query.iter_mut() {
                commands.entity(entity).despawn_recursive();
            }
        },
        _ => {},
    }
}

/// System to toggle the menu state when the Escape key is pressed.
fn toggle_menu(
    mut commands: Commands,
    keyboard_inputs: Res<Input<KeyCode>>,
    simulation_state: Res<State<MenuState>>,
    font_assets: Res<FontAssets>,
) {
    if keyboard_inputs.just_pressed(KeyCode::Escape) {
        match simulation_state.get() {
            MenuState::Closed => {
                commands.insert_resource(NextState(Some(MenuState::Open)));
                commands.spawn((
                    MenuItem,
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "Tank Game",
                                    TextStyle {
                                        font: font_assets.menu_font.clone(),
                                        font_size: 80.0,
                                        color: Color::RED,
                                        },
                                    ).with_style(Style {
                                        margin: UiRect::all(Val::Px(30.0)),
                                        ..default()
                                    }),
                                );
                                parent.spawn(TextBundle::from_section(
                                    "Spiel Pausiert",
                                    TextStyle {
                                        font: font_assets.menu_font.clone(),
                                        font_size: 50.0,
                                        color: Color::WHITE,
                                        },
                                    ).with_style(Style {
                                        margin: UiRect::all(Val::Px(15.0)),
                                        ..default()
                                    }),
                                );
                                parent.spawn(TextBundle::from_section(
                                    "Escape zum Fortsetzen",
                                    TextStyle {
                                        font: font_assets.menu_font.clone(),
                                        font_size: 50.0,
                                        color: Color::WHITE,
                                        },
                                    ).with_style(Style {
                                        margin: UiRect::all(Val::Px(15.0)),
                                        ..default()
                                    }),
                                );

                                // Fortsetzen Button
                                parent.spawn((
                                    FortsetzenButton,
                                    ButtonBundle {
                                        style: Style {
                                            margin: UiRect::all(Val::Px(15.0)),
                                            ..default()
                                        },
                                        background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                                        ..Default::default()
                                }))
                                .with_children(|parent| {
                                    parent.spawn((MenuItem, TextBundle::from_section(
                                        "Fortsetzen",
                                        TextStyle {
                                            font: font_assets.menu_font.clone(),
                                            font_size: 25.0,
                                            color: Color::WHITE,
                                        },
                                    ).with_style(
                                        Style {
                                            margin: UiRect::all(Val::Px(15.0)),
                                            ..default()
                                        }
                                    )));
                                });

                                // FPS Lock Button
                                parent.spawn((
                                    FPSButton,
                                    ButtonBundle {
                                        style: Style {
                                            margin: UiRect::all(Val::Px(15.0)),
                                            ..default()
                                        },
                                        background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                                        ..Default::default()
                                }))
                                .with_children(|parent| {
                                    parent.spawn((
                                        FPSButton,
                                        TextBundle::from_section(
                                        "Framerate: Locked",
                                        TextStyle {
                                            font: font_assets.menu_font.clone(),
                                            font_size: 25.0,
                                            color: Color::WHITE,
                                        },
                                    ).with_style(
                                        Style {
                                            margin: UiRect::all(Val::Px(15.0)),
                                            ..default()
                                        }
                                    )));
                                });

                                // Quit Button
                                parent.spawn((
                                    QuitButton,
                                    ButtonBundle {
                                        style: Style {
                                            margin: UiRect::all(Val::Px(15.0)),
                                            ..default()
                                        },
                                        background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                                        ..Default::default()
                                }))
                                .with_children(|parent| {
                                    parent.spawn((
                                        QuitButton,
                                        TextBundle::from_section(
                                        "Spiel Beenden",
                                        TextStyle {
                                            font: font_assets.menu_font.clone(),
                                            font_size: 25.0,
                                            color: Color::WHITE,
                                        },
                                    ).with_style(
                                        Style {
                                            margin: UiRect::all(Val::Px(15.0)),
                                            ..default()
                                        }
                                    )));
                                });
                        });
                });
            },
            MenuState::Open => {
                commands.insert_resource(NextState(Some(MenuState::Closed)));
            },
        };
    }
}

fn button_hower(
    mut interaction_query: Query<(&Interaction,&mut BackgroundColor),(Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut background_color) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Hovered => {
                background_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.8);
            },
            Interaction::None => {
                background_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.5);
            },
            _ => {},
        }
    }
}

fn fortsetzen_button(
    mut interaction_query: Query<&Interaction,(Changed<Interaction>, With<FortsetzenButton>)>,
    mut commands: Commands,
) {
    for interaction in interaction_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                commands.insert_resource(NextState(Some(MenuState::Closed)));
            },
            _ => {},
        }
    }
}

fn quit_game_button(
    mut interaction_query: Query<&Interaction,(Changed<Interaction>, With<QuitButton>)>,
) {
    for interaction in interaction_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                std::process::exit(0);
            },
            _ => {},
        }
    }
}

fn toggle_framerate_lock(
    mut interaction_query: Query<(&Interaction,&mut Children),(Changed<Interaction>, With<FPSButton>)>,
    mut framerate_resource: ResMut<FramepaceSettings>,
    mut text_query: Query<&mut Text, With<FPSButton>>,
) {
    for (interaction, children) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                match framerate_resource.limiter {
                    bevy_framepace::Limiter::Auto => {
                        framerate_resource.limiter = bevy_framepace::Limiter::Off;
                        if let Some(text_entity) = children.get(0) {
                            if let Ok(mut text) = text_query.get_mut(*text_entity) {
                                text.sections[0].value = "Framerate: Unlocked".to_string();
                            }
                        }
                    },
                    _ => {
                        framerate_resource.limiter = bevy_framepace::Limiter::Auto;
                        if let Some(text_entity) = children.get(0) {
                            if let Ok(mut text) = text_query.get_mut(*text_entity) {
                                text.sections[0].value = "Framerate: Locked".to_string();
                            }
                        }
                    },
                }
            },
            _ => {},
        }
    }
    
}
