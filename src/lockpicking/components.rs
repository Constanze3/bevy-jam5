use bevy::{ecs::component::{ComponentHooks, StorageType}, prelude::*};


#[derive(Component)]
pub enum Lock {
    BarLock
}


#[derive(Component)]
pub struct Locked;

#[derive(Component)]
pub struct LockPicker {
    pub target: Option<Entity>
}

pub struct LockPickTarget {
    pub picker: Entity,
    pub successful_picks_req: u32,
    pub failed_picks_before_break: u32,
    pub successful_pick_counter: u32,
    pub failed_pick_counter: u32
}

impl Component for LockPickTarget {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        // Whenever this component is removed, or an entity with
        // this component is despawned...
        hooks.on_remove(|mut world, targeted_entity, _component_id|{
            // Grab the data that's about to be removed
            let targetable = world.get::<LockPickTarget>(targeted_entity).unwrap();
            // Track down the entity that's targeting us
            let mut targeting = world.get_mut::<LockPicker>(targetable.picker).unwrap();
            // And clear its target, cleaning up any dangling references
            targeting.target = None;
        });
    }
}
