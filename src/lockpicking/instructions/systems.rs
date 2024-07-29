use bevy::prelude::*;

use crate::player_controller::lockpicking::LockPickEvent;

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
            LockPickingInstructionsUIRoot,
            NodeBundle {
                style: Style {
                    display: Display::Block,
                    margin: UiRect { left: Val::Auto, right: Val::Auto, top: Val::Auto, bottom: Val::Px(35.0) },
                    padding: UiRect::all(Val::Px(15.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                LockPickingInstruction,
                TextBundle {
                    style: Style::default(),
                    text: Text::from_section(
                        "(Click on the green as many times as you can to pick the lock on the bike.)",
                        middle_of_screen_info_text_style(),
                    ),
                    visibility: Visibility::Hidden,
                    ..Default::default()
                },
            ));
        });
}

pub fn update_ui(
    mut event_reader: EventReader<LockPickEvent>,
    mut q_lock_instruction_vis: Query<&mut Visibility, With<LockPickingInstruction>>,
) {
    for event in event_reader.read() {
        for mut vis in q_lock_instruction_vis.iter_mut() {
            match event {
                LockPickEvent::Pick(_) => { *vis = Visibility::Visible; },
                LockPickEvent::StopPick => { *vis = Visibility::Hidden; },
            }
        }
    }
}
