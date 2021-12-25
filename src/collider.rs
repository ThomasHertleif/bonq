use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use bevy_inspector_egui::Inspectable;

#[derive(Debug, Component, Inspectable)]
pub enum Collider {
    Solid,
}

pub fn ball_collision(
    mut ball_query: Query<(&mut crate::Moving, &Transform)>,
    collider_query: Query<(Entity, &Collider, &Transform)>,
) {
    let (mut ball, ball_transform) = match ball_query.get_single_mut() {
        Ok(q) => q,
        Err(_) => return,
    };
    let ball_size = ball_transform.scale.truncate();
    let velocity = &mut ball.velocity;

    // check collision with walls
    for (_entity, collider, transform) in collider_query.iter() {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            // only reflect if the ball's velocity is going in the opposite direction of the
            // collision
            match collision {
                Collision::Left => reflect_x = velocity.x > 0.0,
                Collision::Right => reflect_x = velocity.x < 0.0,
                Collision::Top => reflect_y = velocity.y < 0.0,
                Collision::Bottom => reflect_y = velocity.y > 0.0,
            }

            // reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                velocity.x = -velocity.x;
            }

            // reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                velocity.y = -velocity.y;
            }

            // break if this collide is on a solid, otherwise continue check whether a solid is
            // also in collision
            if let Collider::Solid = *collider {
                break;
            }
        }
    }
}
