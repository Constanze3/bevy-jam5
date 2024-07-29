use bevy::prelude::*;

use crate::player_controller::pick_up::Hand;

use super::components::*;

fn middle_of_screen_info_text_style() -> TextStyle {
    TextStyle {
        font_size: 15.0,
        color: Color::hsl(190.04, 0.8972, 0.4961),
        ..Default::default()
    }
}

pub fn setup_ui(mut commands: Commands) {
    commands
        .spawn((
            PickUpInstructionUiRoot,
            NodeBundle {
                style: Style {
                    display: Display::Block,
                    margin: UiRect { left: Val::Auto, right: Val::Auto, top: Val::Auto, bottom: Val::Px(70.0) },
                    padding: UiRect::all(Val::Px(15.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                PickUpInstruction,
                TextBundle {
                    style: Style::default(),
                    text: Text::from_section(
                        "(Use [MouseLeftClick] to throw, and [MouseRightClick] to drop.)",
                        middle_of_screen_info_text_style(),
                    ),
                    visibility: Visibility::Hidden,
                    ..Default::default()
                },
            ));
        });
}

pub fn update_ui(
    hand: Res<Hand>,
    mut q_instruction_vis: Query<&mut Visibility, With<PickUpInstruction>>,
) {
    for mut vis in q_instruction_vis.iter_mut() {
        if hand.is_empty() {
             *vis = Visibility::Hidden;
        } else {
            *vis = Visibility::Visible;
        }
    }
}
