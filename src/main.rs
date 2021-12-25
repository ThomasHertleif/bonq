use bevy::{core::FixedTimestep, prelude::*};
use bevy_inspector_egui::{Inspectable, WorldInspectorPlugin};

mod collider;
mod wall;

const TIME_STEP: f32 = 1.0 / 60.0;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "bonq".to_string(),
            width: 600.,
            height: 800.,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .register_type::<Moving>()
        .register_type::<NewBall>()
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step((TIME_STEP as f64) * 1_f64))
                .with_system(collider::ball_collision)
                .with_system(update_charge_indicator)
                .with_system(is_ball_still_moving)
                .with_system(launch_ball)
                .with_system(move_the_ball)
                .with_system(charge_ball)
                .with_system(spawn_new_ball),
        )
        .run();
}

#[derive(Component, Inspectable)]
struct Ball;

#[derive(Component, Inspectable, Default, Reflect)]
#[reflect(Component)]
struct NewBall {
    degree: f32,
    velocity: f32,
}

#[derive(Component)]
struct TargetAngle;

#[derive(Component)]
struct ShouldGrow;

#[derive(Component, Inspectable, Default)]
struct StickyBall {
    size: f32,
}

#[derive(Component, Inspectable, Default, Reflect)]
pub struct Moving {
    velocity: Vec2,
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Border
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -250.0, 0.0),
                scale: Vec3::new(600.0, 2.0, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::DARK_GREEN,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Border"));

    // Target angle
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -300.0, 0.0),
                scale: Vec3::new(50.0, 5.0, 1.0),
                rotation: Quat::from_rotation_x(0.),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::hsl(0., 0.8, 0.5),
                ..Default::default()
            },
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert(TargetAngle)
        .insert(Name::new("Target Angle"));

    //Walls
    // Add walls
    let wall_thickness = 30.0;

    wall::spawn(
        Color::YELLOW,
        wall_thickness,
        wall::Side::Top,
        &mut commands,
    );
    wall::spawn(
        Color::RED,
        wall_thickness,
        wall::Side::Bottom,
        &mut commands,
    );
    wall::spawn(
        Color::ORANGE,
        wall_thickness,
        wall::Side::Left,
        &mut commands,
    );
    wall::spawn(
        Color::ORANGE_RED,
        wall_thickness,
        wall::Side::Right,
        &mut commands,
    );
}

const MAX_VELOCITY: f32 = 25.;
const MIN_VELOCITY: f32 = 10.;

fn update_charge_indicator(
    mut indicator: Query<(&mut Transform, &mut Sprite, &mut Visibility), With<TargetAngle>>,
    ball: Query<(&NewBall,)>,
) {
    let (mut transform, mut sprite, mut visibility) = indicator.single_mut();
    let (NewBall { degree, velocity },) = match ball.get_single() {
        Ok(q) => {
            visibility.is_visible = true;
            q
        }
        Err(e) => {
            visibility.is_visible = false;
            return;
        }
    };

    transform.rotation = Quat::from_rotation_z(degree.to_radians());
    sprite.color = Color::hsl(velocity / MAX_VELOCITY * 360., 0.8, 0.8);
}

fn charge_ball(keyboard_input: Res<Input<KeyCode>>, mut ball: Query<(&mut NewBall,)>) {
    if !keyboard_input.pressed(KeyCode::Space) {
        return;
    }
    let (mut ball,) = match ball.get_single_mut() {
        Ok(q) => q,
        Err(e) => {
            info!("No ball for you {}", e);
            return;
        }
    };

    ball.degree = (ball.degree + 1.) % 180.;
    ball.velocity = ((ball.velocity + 0.1) % MAX_VELOCITY).max(MIN_VELOCITY);
}

fn launch_ball(
    keyboard_input: Res<Input<KeyCode>>,
    ball: Query<(Entity, &NewBall), With<NewBall>>,
    mut commands: Commands,
) {
    if !keyboard_input.just_released(KeyCode::Space) {
        return;
    }
    let (new_ball, NewBall { degree, velocity }) = match ball.get_single() {
        Ok(q) => q,
        Err(e) => {
            info!("No ball for you {}", e);
            return;
        }
    };

    info!("very good ball");
    commands
        .entity(new_ball)
        .insert(Moving {
            velocity: shoot(*degree, *velocity),
        })
        .insert(collider::Collider::Sticky)
        .insert(Name::new("Moving Ball"))
        .remove::<NewBall>();
}

fn is_ball_still_moving(
    balls: Query<(Entity, &Moving), (With<Moving>, With<Ball>)>,
    mut commands: Commands,
) {
    for (ball, moving) in balls.iter() {
        if moving.velocity.distance(Vec2::ZERO) <= 0.1 {
            commands
                .entity(ball)
                .remove::<Moving>()
                .remove::<Name>()
                .insert(Name::new("Played Ball"))
                .insert(ShouldGrow);
        }
    }
}

fn move_the_ball(mut balls: Query<(&mut Transform, &mut Moving), (With<Moving>, With<Ball>)>) {
    for (mut transform, mut moving) in balls.iter_mut() {
        transform.translation.x += moving.velocity.x;
        transform.translation.y += moving.velocity.y;

        moving.velocity.x = (moving.velocity.x - 0.1).max(0.);
        moving.velocity.y = (moving.velocity.y - 0.1).max(0.);
    }
}

fn spawn_new_ball(
    all_good: Query<(Entity,), (Or<(With<NewBall>, With<Moving>)>, With<Ball>)>,
    mut commands: Commands,
) {
    if all_good.is_empty() {
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, -300.0, 0.0),
                    scale: Vec3::new(10.0, 10.0, 1.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::LIME_GREEN,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Ball)
            .insert(NewBall {
                degree: 90.,
                velocity: 10.,
            })
            .insert(Name::new("New Ball"));
    }
}

fn shoot(angle: f32, velocity: f32) -> Vec2 {
    let radians = angle.to_radians();
    let x = -1. * radians.cos() * velocity;
    let y = radians.sin() * velocity;

    Vec2::new(x, y)
}

#[test]
fn test_shoot() {
    let vec = shoot(0., 1.);
    assert!(vec.x - -1. <= f32::EPSILON);
    assert!(vec.y - 0. <= f32::EPSILON);

    let vec = shoot(90., 1.);
    assert!(vec.x - 0. <= f32::EPSILON);
    assert!(vec.y - 1. <= f32::EPSILON);

    let vec = shoot(180., 1.);
    assert!(vec.x - 1. <= f32::EPSILON);
    assert!(vec.y - 0. <= f32::EPSILON);
}
