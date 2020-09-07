use crate::{
    components::{Enemy, Hitbox2DComponent, Motion2DComponent, Spaceship},
    space_shooter::CollisionEvent,
    systems::hitbox_collide,
};
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, Join, ReadStorage, System, Write},
    shrev::EventChannel,
};

#[derive(Default)]
pub struct CollisionDetectionSystem;

impl<'s> System<'s> for CollisionDetectionSystem {
    type SystemData = (
        ReadStorage<'s, Enemy>,
        ReadStorage<'s, Spaceship>,
        ReadStorage<'s, Hitbox2DComponent>,
        ReadStorage<'s, Motion2DComponent>,
        ReadStorage<'s, Transform>,
        Entities<'s>,
        Write<'s, EventChannel<CollisionEvent>>,
    );

    fn run(
        &mut self,
        (enemies, spaceships, hitboxes, motions, transforms, entities, mut enemy_collision_event_channel): Self::SystemData,
    ) {
        for (entity_a, transform_a, enemy_a, hitbox_a, motion_a) in
            (&entities, &transforms, &enemies, &hitboxes, &motions).join()
        {
            //check for enemy collisions
            for (entity_b, transform_b, enemy_b, hitbox_b, motion_b) in
                (&entities, &transforms, &enemies, &hitboxes, &motions).join()
            {
                if entity_a == entity_b {
                    continue;
                }

                if hitbox_collide(
                    transform_a.translation().x,
                    transform_a.translation().y,
                    transform_b.translation().x,
                    transform_b.translation().y,
                    hitbox_a.width,
                    hitbox_a.height,
                    hitbox_b.width,
                    hitbox_b.height,
                    hitbox_a.offset_x,
                    hitbox_a.offset_y,
                    hitbox_b.offset_x,
                    hitbox_b.offset_y,
                ) {
                    enemy_collision_event_channel.single_write(CollisionEvent::new(
                        entity_a,
                        String::from("enemy"),
                        motion_b.velocity.x,
                        motion_b.velocity.y,
                        entity_b,
                        String::from("enemy"),
                        motion_a.velocity.x,
                        motion_a.velocity.y,
                    ));
                }
            }

            //check for spaceship collisions
            for (entity_b, transform_b, spaceship_b, hitbox_b, motion_b) in
                (&entities, &transforms, &spaceships, &hitboxes, &motions).join()
            {
                if hitbox_collide(
                    transform_a.translation().x,
                    transform_a.translation().y,
                    transform_b.translation().x,
                    transform_b.translation().y,
                    hitbox_a.width,
                    hitbox_a.height,
                    hitbox_b.width,
                    hitbox_b.height,
                    hitbox_a.offset_x,
                    hitbox_a.offset_y,
                    hitbox_b.offset_x,
                    hitbox_b.offset_y,
                ) {
                    enemy_collision_event_channel.single_write(CollisionEvent::new(
                        entity_a,
                        String::from("enemy"),
                        motion_a.velocity.x,
                        motion_a.velocity.y,
                        entity_b,
                        String::from("spaceship"),
                        motion_b.velocity.x,
                        motion_b.velocity.y,
                    ));
                }
            }
        }
    }
}
