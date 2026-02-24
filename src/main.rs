use glam::Vec2;
use tangine::{
    api::Engine,
    ecs::{Friction, MaxSpeed, Position, Sprite, Tint, Velocity},
    input::{InputState, KeyCode},
    render::camera::Camera,
    schedule::Stage,
};

use crate::ecs::components::{Bounded, PlayerEntity, PlayerState, PlayerStateEnum};

pub mod ecs;

pub struct FpsDebug {
    pub elapsed: f32,
    pub frames: u32,
}

fn main() {
    let mut engine = Engine::new();

    engine.on_setup(|world, resources| {
        resources.insert(FpsDebug {
            elapsed: 0.0,
            frames: 0,
        });

        let player = world.spawn((
            Position(Vec2::new(0.0, 0.0)),
            Velocity(Vec2::ZERO),
            Friction(0.97),
            MaxSpeed(1000.0),
            PlayerState {
                current: PlayerStateEnum::Idle,
                previous: PlayerStateEnum::Idle,
            },
            Sprite::from_static("ship_0"),
        ));

        resources.insert(PlayerEntity(player));

        use rand::Rng;
        let mut rng = rand::rng();
        for _ in 0..1000 {
            let x = rng.random_range(-1000.0..1000.0);
            let y = rng.random_range(-1000.0..1000.0);
            world.spawn((
                Position(Vec2::new(x, y)),
                Velocity(Vec2::new(
                    rng.random_range(-200.0..200.0),
                    rng.random_range(-200.0..200.0),
                )),
                Bounded,
                Sprite::from_static("ship_0"),
                Tint {
                    r: rng.random_range(0.0f32..1.0f32),
                    g: rng.random_range(0.0f32..1.0f32),
                    b: rng.random_range(0.0f32..1.0f32),
                    ..Default::default()
                },
            ));
        }
    });

    engine
        .schedule_mut()
        .add_system(Stage::PrePhysics, move |world, resources, time| {
            let input = resources.get::<InputState>().unwrap();
            let vel_increase = 800.0 * time.delta;
            let player = resources.get::<PlayerEntity>().unwrap().0;
            let (vel, state) = world
                .query_one_mut::<(&mut Velocity, &mut PlayerState)>(player)
                .unwrap();

            let mut moving = false;
            if input.is_pressed(KeyCode::KeyW) {
                vel.0.y -= vel_increase;
                moving = true;
            }
            if input.is_pressed(KeyCode::KeyS) {
                vel.0.y += vel_increase;
                moving = true;
            }
            if input.is_pressed(KeyCode::KeyA) {
                vel.0.x -= vel_increase;
                moving = true;
            }
            if input.is_pressed(KeyCode::KeyD) {
                vel.0.x += vel_increase;
                moving = true;
            }

            state.previous = state.current;
            if moving {
                state.current = PlayerStateEnum::Flying;
            } else {
                state.current = PlayerStateEnum::Idle;
            }
        });

    engine
        .schedule_mut()
        .add_system(Stage::PrePhysics, |world, _resource, _time| {
            for (pos, vel, _bounded) in world.query_mut::<(&Position, &mut Velocity, &Bounded)>() {
                if pos.0.x <= -1000.0 && vel.0.x < 0.0 {
                    vel.0.x *= -1.0;
                } else if pos.0.x >= 1000.0 && vel.0.x > 0.0 {
                    vel.0.x *= -1.0;
                }
                if pos.0.y <= -1000.0 && vel.0.y < 0.0 {
                    vel.0.y *= -1.0;
                } else if pos.0.y >= 1000.0 && vel.0.y > 0.0 {
                    vel.0.y *= -1.0;
                }
            }
        });

    engine
        .schedule_mut()
        .add_system(Stage::PreRender, move |world, resources, time| {
            let player = resources.get::<PlayerEntity>().unwrap().0;
            let (state, mut _animation) = world
                .query_one_mut::<(&PlayerState, &mut Sprite)>(player)
                .unwrap();

            if state.previous != PlayerStateEnum::Flying && state.current == PlayerStateEnum::Flying
            {
                _animation.set_animated(vec!["ship_1", "ship_2"], 4.0, true, time.elapsed);
            } else if state.previous != PlayerStateEnum::Idle
                && state.current == PlayerStateEnum::Idle
            {
                _animation.set_static("ship_0");
            }
        });

    engine
        .schedule_mut()
        .add_system(Stage::PreRender, |world, resources, _time| {
            let player = resources.get::<PlayerEntity>().unwrap().0;
            let pos = world.query_one_mut::<&Position>(player).unwrap();
            let camera = resources.get_mut::<Camera>().unwrap();
            camera.position = pos.0 - camera.viewport * 0.5;
        });

    engine
        .schedule_mut()
        .add_system(Stage::UIUpdate, |_world, resources, time| {
            if let Some(fps_debug) = resources.get_mut::<FpsDebug>() {
                fps_debug.elapsed += time.delta;
                fps_debug.frames += 1;

                if fps_debug.elapsed >= 5.0 {
                    let fps = fps_debug.frames / 5u32;
                    println!("{} fps", fps);
                    fps_debug.elapsed = 0.0;
                    fps_debug.frames = 0;
                }
            }
        });

    engine.run();
}
