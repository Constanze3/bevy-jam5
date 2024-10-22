use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};

/// Ui settings for sliding
pub enum SlideSettings {
    NoSlide,
    SlideLinear(SlideLinear),
}

// TODO move this into Slide settings
pub struct SlideLinear {
    pub speed: f32,
    pub time_to_target: f32,
}

/// Data of a slide of a widget.
#[derive(Component)]
pub struct SlideTarget {
    pub speed: f32,
    pub start_pos: f32,
    pub target_pos: f32,
    pub time_to_target: f32,
    pub start_time_secs: f32,
}

#[derive(Component)]
pub struct RandomizePos;

/// Marks object that can be lockpicked, `Locked` is removed on a successful lockpick.
pub struct Locked {
    pub success_zone_width: f32,
    pub move_on_good_pick: bool,
    pub zone_slide_settings: SlideSettings,
}

impl Component for Locked {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_remove(|mut world, e, _component_id| {
            world.commands().entity(e).remove::<LockPickTarget>();
        });
    }
}

/// Marks an entity that can pick locks i.e. the player.
#[derive(Component, Default)]
pub struct LockPicker {
    pub target: Option<Entity>,
}

/// Data about a lockpicking session.
pub struct LockPickTarget {
    pub picker: Entity,
    pub successful_picks_before_unlock: u32,
    pub failed_picks_before_break: u32,
    pub successful_pick_counter: u32,
    pub failed_pick_counter: u32,
}

impl Component for LockPickTarget {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        // Whenever this component is removed, or an entity with
        // this component is despawned...
        hooks.on_remove(|mut world, targeted_entity, _component_id| {
            // Grab the data that's about to be removed
            let targetable = world.get::<LockPickTarget>(targeted_entity).unwrap();
            // Track down the entity that's targeting us
            let mut targeting = world.get_mut::<LockPicker>(targetable.picker).unwrap();
            // And clear its target, cleaning up any dangling references
            targeting.target = None;
        });
    }
}
