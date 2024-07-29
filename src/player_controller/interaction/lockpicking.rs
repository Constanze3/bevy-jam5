use avian3d::dynamics::rigid_body::RigidBody;
use bevy::prelude::*;

use crate::{
    lockpicking::*,
    player_controller::{CharacterController, Player},
};

use super::interact;

#[derive(Event)]
pub struct LockPickEvent(pub Entity);

pub fn plugin(app: &mut App) {
    app.add_event::<LockPickEvent>()
        .add_systems(Update, lockpick.after(interact));
}

fn lockpick(
    mut lock_pick_er: EventReader<LockPickEvent>,
    mut q_player: Query<(Entity, &LockPicker, &mut CharacterController), With<Player>>,
    mut q_lock: Query<&mut RigidBody>,
    mut commands: Commands,
) {
    for ev in lock_pick_er.read() {
        let entity = ev.0;
        let (picker_entity, picker, mut character_controller) = q_player.get_single_mut().unwrap();

        // dont overwrite lock picking if its already in progress!
        if picker.target.is_some() {
            return;
        }

        character_controller.locked = true;

        let mut rigidbody = q_lock.get_mut(entity).unwrap();
        *rigidbody = RigidBody::Static;

        commands.entity(entity).insert(LockPickTarget {
            picker: picker_entity,
            successful_pick_counter: 0,
            failed_pick_counter: 0,
            successful_picks_before_unlock: 3,
            failed_picks_before_break: 1,
        });
    }
}
