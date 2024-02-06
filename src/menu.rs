use bevy::prelude::*;

use crate::asset_loader::FontAssets;


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
            .add_systems(First, toggle_menu)
            .add_systems(Update, toggle_display_menu);
    }
}

/// System to toggle the menu state when the Escape key is pressed.
fn toggle_menu(
    mut commands: Commands,
    keyboard_inputs: Res<Input<KeyCode>>,
    simulation_state: Res<State<MenuState>>,
) {
    if keyboard_inputs.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(Some( match simulation_state.get() {
            MenuState::Open => MenuState::Closed,
            MenuState::Closed => MenuState::Open,
        })));
    }
}

/// System to display or hide the menu based on the current menu state.
/// If the menu state is `MenuState::Open`, the menu is displayed.
/// If the menu state is `MenuState::Closed`, the menu is hidden.
fn toggle_display_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    simulation_state: Res<State<MenuState>>,
    mut query: Query<(Entity, &MenuItem)>,
) {
    if simulation_state.is_changed() {
        match simulation_state.get() {
            MenuState::Open => {
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
                        });
                });
            },
            MenuState::Closed => {
                for (entity, _) in query.iter_mut() {
                    commands.entity(entity).despawn_recursive();
                }
            },
        }
        
    }
}