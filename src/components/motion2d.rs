use amethyst::{
    core::math::Vector2,
    ecs::prelude::{Component, DenseVecStorage},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Motion2DComponent {
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub deceleration: Vector2<f32>,
    // x/y speed an entity can accelerate to
    pub speed: Vector2<f32>,
    // Max x/y speed an entity can reach
    pub max_speed: Vector2<f32>,
    pub angular_velocity: f32,
    pub angular_acceleration: f32,
    pub angular_deceleration: f32,
    pub angular_speed: f32,
    pub immovable: bool, // can't be moved from outside forces
    pub target_position: Option<Vector2<f32>>,
}

impl Component for Motion2DComponent {
    type Storage = DenseVecStorage<Self>;
}

impl Motion2DComponent {
    // accelerate to speed stat in negative y direction
    // decelerate if over speed stat
    pub fn move_down(&mut self) {
        if self.velocity.y.abs() < self.speed.y {
            self.velocity.y -= self.acceleration.y;
        } else if self.velocity.y.abs() >= self.speed.y {
            if self.velocity.y > 0.0 {
                self.velocity.y -= self.deceleration.y;
            } else {
                self.velocity.y += self.deceleration.y;
            }
        }
    }

    // accelerate to speed stat in positive y direction
    // decelerate if over speed stat
    pub fn move_up(&mut self) {
        if self.velocity.y.abs() < self.speed.y {
            self.velocity.y += self.acceleration.y;
        } else if self.velocity.y.abs() >= self.speed.y {
            if self.velocity.y > 0.0 {
                self.velocity.y -= self.deceleration.y;
            } else {
                self.velocity.y += self.deceleration.y;
            }
        }
    }

    // decelerate if moving in the x direction
    pub fn brake_horizontal(&mut self) {
        if self.velocity.x > 0.0 {
            self.velocity.x -= self.deceleration.x;
        } else if self.velocity.x < 0.0 {
            self.velocity.x += self.deceleration.x;
        }
    }

    // turn to face the target
    pub fn turn_towards_target(&mut self, current_position: Vector2<f32>, current_angle: f32) {
        if let Some(target_position) = self.target_position {
            let target_angle = (current_position.y - target_position.y)
                .atan2(current_position.x - target_position.x)
                .to_degrees()
                + 180.0;

            let adjusted_angle = current_angle + 90.0;

            let smallest_angle =
                signed_modulo(target_angle - adjusted_angle + 180.0, 360.0) - 180.0;

            if smallest_angle >= 0.0 {
                self.angular_velocity -= self.angular_acceleration;
            } else {
                self.angular_velocity += self.angular_acceleration;
            }
        }
    }

    // move in direction that the entity is facing
    pub fn move_forward(&mut self, angle: f32) {
        if self.velocity.x < self.speed.x * (angle - std::f32::consts::FRAC_PI_2).cos() {
            self.velocity.x += self.acceleration.x;
        } else {
            self.velocity.x -= self.acceleration.x;
        }

        if self.velocity.y < self.speed.y * (angle - std::f32::consts::FRAC_PI_2).sin() {
            self.velocity.y += self.acceleration.y;
        } else {
            self.velocity.y -= self.acceleration.y;
        }
    }
}

fn signed_modulo(a: f32, n: f32) -> f32 {
    a - (a / n).floor() * n
}
