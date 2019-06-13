use amethyst::{
    core::{
        Transform,
        timing::Time,
        nalgebra::Vector3,
    },
    ecs::{Join, Read, System, WriteStorage, Entities, LazyUpdate, ReadExpect},
};

use rand::{thread_rng, Rng};

use crate::{
    entities::{spawn_enemy},
    components::{Spawner, Enemy},
    resources::SpriteResource,
};
use crate::space_shooter::{ARENA_MAX_X, ARENA_MIN_X, ARENA_SPAWN_OFFSET};


pub struct SpawnerSystem;

impl<'s> System<'s> for SpawnerSystem {

    type SystemData  = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Spawner<Enemy>>,
        Read<'s, Time>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut transforms, mut spawners, time, enemy_resource, lazy_update): Self::SystemData) {

        for (spawner, transform) in (&mut spawners, &mut transforms).join() {
            if spawner.can_spawn(time.delta_seconds()) {

                let max_width = ARENA_MAX_X - ARENA_SPAWN_OFFSET;
                let min_width = ARENA_MIN_X + ARENA_SPAWN_OFFSET;
                let new_x = ARENA_MIN_X + ARENA_SPAWN_OFFSET + thread_rng().gen::<f32>()* (max_width - min_width);

                let spawn_position = Vector3::new(
                    new_x, transform.translation()[1], transform.translation()[2],
                );

                spawn_enemy(&entities, &enemy_resource, &mut spawner.pool, spawn_position, &lazy_update);
            }
        }
    }
}