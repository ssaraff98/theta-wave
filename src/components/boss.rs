use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};

use crate::components::Enemy;

pub struct Repeater {
    pub body: Entity,
    pub head: Entity,
    pub right_shoulder: Entity,
    pub left_shoulder: Entity,
    //pub right_arm: Entity,
}

impl Component for Repeater {
    type Storage = DenseVecStorage<Self>;
}
