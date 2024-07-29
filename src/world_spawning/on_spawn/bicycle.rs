use crate::*;
use avian3d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

use self::lockpicking::{Locked, SlideLinear, SlideSettings};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, spawn.run_if(in_state(GameState::Playing)));
}

#[derive(Component)]
pub struct Bicycle;

#[derive(Component)]
struct SpawnTimer(Timer);

fn spawn(q_bicycle: Query<Entity, Added<Bicycle>>, mut commands: Commands) {
    for bicycle_entity in q_bicycle.iter() {
        commands
            .entity(bicycle_entity)
            .insert(RigidBody::Dynamic)
            .with_children(|parent| {
                parent.spawn(Collider::cuboid(1.7, 1.0, 0.2));
                parent.spawn((
                    Collider::cuboid(0.1, 0.1, 0.4),
                    TransformBundle::from_transform(Transform::from_xyz(0.35, 0.4, 0.0)),
                ));
            })
            .insert(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Once)));
    }
}

fn after_spawn(
    time: Res<Time>,
    mut q_bicycle: Query<(Entity, &mut SpawnTimer, &mut RigidBody), With<Bicycle>>,
    mut commands: Commands,
) {
    for (bicycle_entity, mut spawn_timer, mut rigidbody) in q_bicycle.iter_mut() {
        spawn_timer.0.tick(time.delta());

        if spawn_timer.0.finished() {
            *rigidbody = RigidBody::Static;

            commands
                .entity(bicycle_entity)
                .insert(Locked {
                    success_zone_width: 10.0,
                    move_on_good_pick: true,
                    zone_slide_settings: SlideSettings::SlideLinear(SlideLinear {
                        speed: 10.0,
                        time_to_target: 1.5,
                    }),
                })
                .remove::<SpawnTimer>();
        }
    }
}
