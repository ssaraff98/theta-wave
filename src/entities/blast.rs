use amethyst::ecs::prelude::{World, Entities, Entity, LazyUpdate, ReadExpect};
use amethyst::core::transform::Transform;
use amethyst::core::nalgebra::Vector3;
use amethyst::renderer::{SpriteSheetHandle, SpriteRender};

use crate::components::Blast;
use crate::resources::BlastResource;


const BLAST_STARTING_SPEED: f32 = 100.0;


pub fn initialise_blast_resource(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) -> BlastResource {
    let blast_resource = BlastResource {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1,
    };

    world.add_resource(blast_resource.clone());
    blast_resource
}

pub fn fire_blast(entities: &Entities, blast_resource: &ReadExpect<BlastResource>, fire_position: Vector3<f32>, lazy_update: &ReadExpect<LazyUpdate>) {
    let blast_entity: Entity = entities.create();

    let mut local_transform = Transform::default();
    local_transform.set_position(fire_position);

    let sprite_render = SpriteRender {
        sprite_sheet: blast_resource.sprite_sheet.clone(),
        sprite_number: blast_resource.sprite_number,
    };

    lazy_update.insert(blast_entity, sprite_render);
    lazy_update.insert(blast_entity, Blast {speed: BLAST_STARTING_SPEED});
    lazy_update.insert(blast_entity, local_transform);

}