use crate::{
    components::{
        BarrelRollAbilityComponent, DefenseTag, HealthComponent, PlayerComponent,
        StatusBarComponent, StatusType,
    },
    entities::spawn_status_unit,
    resources::{SpriteSheetsResource, StoreResource},
};
use amethyst::ecs::prelude::{
    Entities, Join, LazyUpdate, ReadExpect, ReadStorage, System, WriteStorage,
};

const HEALTH_SPRITE_INDEX: usize = 0;
const DEFENSE_SPRITE_INDEX: usize = 1;
const ROLL_SPRITE_INDEX: usize = 2;
const RESTOCK_SPRITE_INDEX: usize = 3;

pub struct StatusBarSystem;

impl<'s> System<'s> for StatusBarSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, StatusBarComponent>,
        ReadStorage<'s, PlayerComponent>,
        ReadStorage<'s, BarrelRollAbilityComponent>,
        ReadStorage<'s, DefenseTag>,
        ReadStorage<'s, HealthComponent>,
        ReadExpect<'s, StoreResource>,
        ReadExpect<'s, SpriteSheetsResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut status_bars,
            players,
            barrel_roll_abilities,
            defense_tags,
            healths,
            store_resource,
            sprite_resource,
            lazy_update,
        ): Self::SystemData,
    ) {
        for status_bar in (&mut status_bars).join() {
            match status_bar.status_type {
                StatusType::Health => {
                    for (_player, health) in (&players, &healths).join() {
                        if let Some(status_position) =
                            status_bar.update_units_y(health.max_value, health.value, &entities)
                        {
                            status_bar.status_unit_stack.push(spawn_status_unit(
                                &entities,
                                &sprite_resource,
                                HEALTH_SPRITE_INDEX,
                                status_position,
                                &lazy_update,
                            ));
                        }
                    }
                }

                StatusType::Defense => {
                    for (_defense_tag, defense_health) in (&defense_tags, &healths).join() {
                        if let Some(status_position) = status_bar.update_units_y(
                            defense_health.max_value,
                            defense_health.value,
                            &entities,
                        ) {
                            status_bar.status_unit_stack.push(spawn_status_unit(
                                &entities,
                                &sprite_resource,
                                DEFENSE_SPRITE_INDEX,
                                status_position,
                                &lazy_update,
                            ));
                        }
                    }
                }

                StatusType::Roll => {
                    for barrel_roll_ability in (&barrel_roll_abilities).join() {
                        if let Some(status_position) = status_bar.update_units_x(
                            barrel_roll_ability.execute_cooldown,
                            barrel_roll_ability.execute_cooldown
                                - barrel_roll_ability.execute_timer,
                            &entities,
                        ) {
                            status_bar.status_unit_stack.push(spawn_status_unit(
                                &entities,
                                &sprite_resource,
                                ROLL_SPRITE_INDEX,
                                status_position,
                                &lazy_update,
                            ));
                        }
                    }
                }

                StatusType::Restock => {
                    if let Some(status_position) = status_bar.update_units_x(
                        store_resource.restock_period,
                        store_resource.restock_period - store_resource.restock_timer,
                        &entities,
                    ) {
                        status_bar.status_unit_stack.push(spawn_status_unit(
                            &entities,
                            &sprite_resource,
                            RESTOCK_SPRITE_INDEX,
                            status_position,
                            &lazy_update,
                        ));
                    }
                }
            }
        }
    }
}
