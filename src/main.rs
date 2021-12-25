use bevy::{core::FixedTimestep, prelude::*};
use bevy_inspector_egui::{Inspectable, WorldInspectorPlugin};
use std::f32::consts::PI;

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
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(launch_ball)
                .with_system(move_the_ball),
        )
        .run();
}

#[derive(Component, Inspectable)]
struct Ball;

#[derive(Component, Inspectable, Default)]
struct NewBall {
    degree: f32,
    velocity: f32,
}

#[derive(Component, Inspectable, Default)]
struct StickyBall {
    size: f32,
}

#[derive(Component, Inspectable, Default)]
struct Moving {
    velocity: Vec2,
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Border
    commands.spawn_bundle(SpriteBundle {
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
    });

    // NewBall
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
            degree: 360.,
            velocity: 1.,
        });

    //Walls
    // Add walls
    let wall_thickness = 10.0;

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

fn launch_ball(
    keyboard_input: Res<Input<KeyCode>>,
    ball: Query<(Entity, &NewBall), With<NewBall>>,
    mut commands: Commands,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        if let Ok((new_ball, NewBall { degree, velocity })) = ball.get_single() {
            info!("very good ball");
            commands
                .entity(new_ball)
                .insert(Moving {
                    velocity: {
                        let radians = degree / (180. * PI);
                        let x = radians.cos() * velocity;
                        let y = radians.sin() * velocity;

                        Vec2::new(x, y)
                    },
                })
                .remove::<NewBall>();
        } else {
            info!("No ball for you");
        }
    }
}

fn move_the_ball(mut balls: Query<(&mut Transform, &Moving), (With<Moving>, With<Ball>)>) {
    for (mut transform, Moving { velocity }) in balls.iter_mut() {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}
