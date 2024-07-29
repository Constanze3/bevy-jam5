use crate::player_controller::{lockpicking::LockPickEvent, pick_up::UpPickable, CharacterController};

use super::*;

use avian3d::dynamics::rigid_body::RigidBody;
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
                    // println!("failed pick");
                    target.failed_pick_counter += 1;

                    if target.failed_picks_before_break < target.failed_pick_counter {
                        // println!("oops, lockpick broke..");
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
                    // println!("succeded pick");
                    target.successful_pick_counter += 1;

                    if target.successful_picks_before_unlock < target.successful_pick_counter {
                        // println!("successful unlock!");
                        commands
                            .entity(target_entity)
                            .remove::<Locked>()
                            .remove::<LockPickTarget>();
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

pub fn on_remove_lockpick_target(
    mut removals: RemovedComponents<LockPickTarget>,
    mut lockpick_event_writer: EventWriter<LockPickEvent>,
    mut q_object: Query<&mut RigidBody>,
    mut q_player: Query<&mut CharacterController>,
) {
    for entity in removals.read() {
        lockpick_event_writer.send(LockPickEvent::StopPick);
        let mut character_controller = q_player.get_single_mut().unwrap();
        character_controller.locked = false;

        let mut rigidbody = q_object.get_mut(entity).unwrap();
        *rigidbody = RigidBody::Dynamic;
    }
}

pub fn on_remove_lock(mut removals: RemovedComponents<Locked>, mut commands: Commands) {
    for entity in removals.read() {
        commands.entity(entity).insert(UpPickable);
    }
}
