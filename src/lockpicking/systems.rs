use crate::player_controller::Player;

use super::*;
use avian3d::prelude::*;

use bevy::prelude::*;
use bevy_camera_extras::CameraControls;


pub fn check_fail_clicks(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<PickFailZone>)>,
    mut lockpick_targets: Query<(Entity, &mut LockPickTarget)>,
    mut commands: Commands
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
            Interaction::None => {},
            Interaction::Hovered => println!("hoverd over fail button")
        }
    }
}

pub fn check_success_clicks(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<PickSuccessZone>)>,
    mut lockpick_targets: Query<&mut LockPickTarget>
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                for mut target in lockpick_targets.iter_mut() {
                    println!("succeded pick");
                    target.successful_pick_counter += 1;
                }
            }
            Interaction::None => {},
            Interaction::Hovered => {}
        }
    }
}

pub fn check_for_lockpick_request(
    mut pickers: Query<(Entity, &mut LockPicker, &Transform), Without<Locked>>,
    locked_locks: Query<(Entity, &Transform), (With<Locked>, Without<LockPickTarget>)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if keys.pressed(KeyCode::KeyE) {
        for (e, mut picker, picker_trans) in pickers.iter_mut() {
            
            // dont overwrite lock picking if its already in progress!
            if picker.target.is_some() {return}

            let locks = locked_locks.iter()
            .map(|(lock, trans)| (lock, trans.translation)).collect::<Vec<_>>();
    
            let Some(first_lock) = locks.first() else {return};
    
            let closest_lock = *locks.iter().fold(first_lock, |current, candidate| {
                if current.1.distance(picker_trans.translation) > candidate.1.distance(picker_trans.translation) {
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
                successful_picks_req: 3,
                failed_picks_before_break: 1
            });
            println!("attempting to pick {:#?}", picker.target)
        }

    }
}

// fn interact(
//     keys: Res<ButtonInput<KeyCode>>,
//     query: SpatialQuery,
//     q_camera: Query<&Transform, With<CameraControls>>,
//     players: Query<(Entity, &LockPicker), Without<CameraControls>>,
// ) {
//     if keys.just_pressed(KeyCode::KeyE) {
//         let transform = q_camera.get_single().unwrap();

//         let origin = transform.translation;
//         let direction = transform.forward();

//         let Some(hit) = query.cast_ray_predicate(
//             origin,
//             direction,
//             100.0,
//             true,
//             SpatialQueryFilter::default(),
//             &|entity| q_entities.get(entity).unwrap().1.is_none(),
//         ) else {
//             return;
//         };

//         let (entity, _, up_pickable) = q_entities.get(hit.entity).unwrap();

//         if up_pickable.is_some() {
//             pick_up_ew.send(PickUpEvent(entity));
//         }
//     }
// }
