use amethyst::prelude::Builder;
use amethyst::ecs::prelude::{World};
use amethyst::core::transform::Transform;
use amethyst::renderer::{SpriteRender, SpriteSheetHandle};

use crate::components::Spaceship;
use crate::space_shooter::{GAME_HEIGHT, GAME_WIDTH};


const SPACESHIP_HEIGHT: f32 = 18.0;
const SPACESHIP_WIDTH: f32 = 18.0;
const SPACESHIP_STARTING_SPEED: f32 = 70.0;
const SPACESHIP_STARTING_FIRE_SPEED: f32 = 0.5;


pub fn initialise_spaceship(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {

    //create transform
    let mut local_transform = Transform::default();
    local_transform.set_xyz(GAME_WIDTH / 2.0, GAME_HEIGHT / 6.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Spaceship {
            width: SPACESHIP_WIDTH,
            height: SPACESHIP_HEIGHT,
            speed: SPACESHIP_STARTING_SPEED,
            fire_speed: SPACESHIP_STARTING_FIRE_SPEED,
            fire_reset_timer: 0.0,
        })
        .with(local_transform)
        .build();
}