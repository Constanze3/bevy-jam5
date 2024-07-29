use super::*;

use bevy::prelude::*;

pub fn check_fail_clicks(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<PickFailZone>),
    >,
    mut lockpick_targets: Query<(Entity, &mut LockPickTarget)>,
    mut commands: Commands,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                for (e, mut target) in lockpick_targets.iter_mut() {
                    println!("failed pick");
                    target.failed_pick_counter += 1;

                    if target.failed_picks_before_break < target.failed_pick_counter {
                        println!("oops, lockpick broke..");
                        commands.entity(e).remove::<LockPickTarget>();
                    }
                }
            }
            Interaction::None => {}
            Interaction::Hovered => {}
        }
    }
}

pub fn check_success_clicks(
    mut interaction_query: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<Button>, With<PickSuccessZone>),
    >,
    mut lockpick_targets: Query<(Entity, &Locked, &mut LockPickTarget)>,
    mut commands: Commands,
) {
    for (button_entity, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                for (target_entity, lock_setings, mut target) in lockpick_targets.iter_mut() {
                    println!("succeded pick");
                    target.successful_pick_counter += 1;

                    if target.successful_picks_before_unlock < target.successful_pick_counter {
                        println!("successful unlock!");
                        commands.entity(target_entity).remove::<Locked>();
                    }

                    if lock_setings.move_on_good_pick {
                        commands.entity(button_entity).insert(RandomizePos);
                    }
                }
            }
            Interaction::None => {}
            Interaction::Hovered => {}
        }
    }
}

// TODO move this to interaction
pub fn check_for_lockpick_request(
    mut pickers: Query<(Entity, &mut LockPicker, &Transform), Without<Locked>>,
    locked_locks: Query<(Entity, &Transform), (With<Locked>, Without<LockPickTarget>)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if keys.pressed(KeyCode::KeyE) {
        for (e, mut picker, picker_trans) in pickers.iter_mut() {
            // dont overwrite lock picking if its already in progress!
            if picker.target.is_some() {
                return;
            }

            let locks = locked_locks
                .iter()
                .map(|(lock, trans)| (lock, trans.translation))
                .collect::<Vec<_>>();

            let Some(first_lock) = locks.first() else {
                return;
            };

            let closest_lock = *locks.iter().fold(first_lock, |current, candidate| {
                if current.1.distance(picker_trans.translation)
                    > candidate.1.distance(picker_trans.translation)
                {
                    candidate
                } else {
                    current
                }
            });

            picker.target = Some(closest_lock.0);

            commands.entity(closest_lock.0).insert(LockPickTarget {
                picker: e,
                successful_pick_counter: 0,
                failed_pick_counter: 0,
                successful_picks_before_unlock: 3,
                failed_picks_before_break: 1,
            });
            println!("attempting to pick {:#?}", picker.target)
        }
    }
}
