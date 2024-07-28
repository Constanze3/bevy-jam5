use avian3d::collision::CollidingEntities;
use bevy::prelude::*;

use crate::{
    car_controller::{components::Sticky, CarAction, CarController},
    points::PointsAction,
    world_spawning::on_spawn::Illegal,
    GameState,
};

pub fn plugin(app: &mut App) {
    app.add_event::<DropOffBicyclesEvent>().add_systems(
        Update,
        (on_enter, drop_off_bicycles)
            .chain()
            .run_if(in_state(GameState::Playing)),
    );
}

#[derive(Component)]
pub struct Home;

fn on_enter(
    q_home: Query<&CollidingEntities, (With<Home>, Changed<CollidingEntities>)>,
    q_child: Query<Option<&Parent>>,
    q_car_controller: Query<(Option<&CarController>, Option<&Children>)>,
    q_sticky: Query<Option<&Sticky>>,
    mut drop_off_bicycles_ew: EventWriter<DropOffBicyclesEvent>,
    mut car_action_ew: EventWriter<CarAction>,
) {
    for colliding_entities in q_home.iter() {
        for colliding_entity in colliding_entities.iter() {
            let parent = q_child.get(*colliding_entity).unwrap();

            if let Some(parent) = parent {
                let parent_entity = parent.get();

                let (car_controller, children) = q_car_controller.get(parent_entity).unwrap();
                if car_controller.is_some() {
                    car_action_ew.send(CarAction::Refuel);

                    let children = children.unwrap();

                    let mut sticky_entities = Vec::new();
                    for child_entity in children {
                        if let Some(sticky) = q_sticky.get(*child_entity).unwrap() {
                            if !sticky.entities.is_empty() {
                                sticky_entities.push(*child_entity);
                            }
                        }
                    }

                    if !sticky_entities.is_empty() {
                        drop_off_bicycles_ew.send(DropOffBicyclesEvent(sticky_entities));
                    }
                }
            }
        }
    }
}

#[derive(Event)]
pub struct DropOffBicyclesEvent(Vec<Entity>);

fn drop_off_bicycles(
    mut er: EventReader<DropOffBicyclesEvent>,
    mut q_sticky: Query<(&mut Sticky, &mut CollidingEntities)>,
    q_bicycle: Query<Option<&Illegal>>,
    mut points_action_ew: EventWriter<PointsAction>,
    mut commands: Commands,
) {
    for ev in er.read() {
        for sticky_entity in &ev.0 {
            let (mut sticky, mut colliding_entities) = q_sticky.get_mut(*sticky_entity).unwrap();

            for fake_bicycle_entity in &sticky.entities {
                let illegal = q_bicycle.get(*fake_bicycle_entity).unwrap();
                if illegal.is_some() {
                    points_action_ew.send(PointsAction::Increment(1));
                } else {
                    // subtract a points
                    points_action_ew.send(PointsAction::Increment(2));
                }

                commands.entity(*fake_bicycle_entity).despawn_recursive();
            }

            sticky.entities.clear();
            colliding_entities.clear();
        }

        return;
    }
}
