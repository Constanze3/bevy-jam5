use bevy::prelude::*;

use super::*;

pub struct LockpickingPlugin;

impl Plugin for LockpickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                randomize_lockpick_zone_position,
                slide_sliding_pick_zones,
                spawn_lockpicking_minigame_ui,
                despawn_lockpicking_minigame_ui,
                adjust_lockpick_position,
                check_fail_clicks,
                check_success_clicks,
            )
                .chain(),
        );
        app.add_systems(
            Update,
            (
                on_remove_lock.after(check_success_clicks),
                on_remove_lockpick_target
                    .after(check_success_clicks)
                    .after(check_fail_clicks),
            ),
        );
    }
}