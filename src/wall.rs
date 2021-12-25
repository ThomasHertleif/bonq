use bevy::prelude::*;

#[derive(Debug)]
pub enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

pub fn spawn(color: Color, thickness: f32, side: Side, commands: &mut Commands) {
    let (win_w, win_h) = (600., 800.);
    let hor_from_center = (win_h / 2.) - thickness;
    let ver_from_center = (win_w / 2.) - thickness;

    commands
        .spawn_bundle(SpriteBundle {
            transform: match side {
                Side::Top => Transform {
                    translation: Vec3::new(0., hor_from_center, 0.0),
                    scale: Vec3::new(win_w - thickness, thickness, 1.0),
                    ..Default::default()
                },
                Side::Bottom => Transform {
                    translation: Vec3::new(0., -hor_from_center, 0.0),
                    scale: Vec3::new(win_w - thickness, thickness, 1.0),
                    ..Default::default()
                },
                Side::Left => Transform {
                    translation: Vec3::new(-ver_from_center, 0., 0.0),
                    scale: Vec3::new(thickness, win_h - thickness, 1.0),
                    ..Default::default()
                },
                Side::Right => Transform {
                    translation: Vec3::new(ver_from_center, 0., 0.0),
                    scale: Vec3::new(thickness, win_h - thickness, 1.0),
                    ..Default::default()
                },
            },
            sprite: Sprite {
                color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new(format!("SideWall{:?}", side)));
    // .insert(Collider::Solid);
}
