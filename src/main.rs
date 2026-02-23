use glam::Vec2;
use tangine::{
    api::Engine,
    ecs::{AnimatedSprite, Friction, MaxSpeed, Player, Position, StaticSprite, Tint, Velocity},
    input::{InputState, KeyCode},
    render::camera::Camera,
    schedule::Stage,
};

use crate::components::components::{PlayerState, PlayerStateEnum};

pub mod components;

pub struct FpsDebug {
    pub elapsed: f32,
    pub frames: u32,
}

fn main() {
    let mut engine = Engine::new();

    engine.resources.insert(FpsDebug {
        elapsed: 0.0,
        frames: 0,
    });

    let player_entity = engine.world_mut().spawn((
        Player,
        Position(Vec2::new(100.0, 100.0)),
        Velocity(Vec2::ZERO),
        Friction(0.97),
        MaxSpeed(1000.0),
        PlayerState {
            current: PlayerStateEnum::Idle,
            previous: PlayerStateEnum::Idle,
        },
        AnimatedSprite {
            frames: vec!["ship_0".into()],
            fps: 1.0,
            looping: true,
            start_time: 0.0,
        },
    ));

    use rand::Rng;
    let mut rng = rand::rng();
    for _ in 0..1000 {
        let x = rng.random_range(0.0f32..2000.0);
        let y = rng.random_range(0.0f32..2000.0);
        engine.world_mut().spawn((
            Position(Vec2::new(x, y)),
            StaticSprite {
                region: "ship_0".into(),
            },
            Tint {
                r: rng.random_range(0.0f32..1.0f32),
                g: rng.random_range(0.0f32..1.0f32),
                b: rng.random_range(0.0f32..1.0f32),
                ..Default::default()
            },
        ));
    }

    engine
        .schedule_mut()
        .add_system(Stage::PrePhysics, move |world, resources, time| {
            let input = resources.get::<InputState>().unwrap();
            let vel_increase = 800.0 * time.delta;
            let (vel, state) = world
                .query_one_mut::<(&mut Velocity, &mut PlayerState)>(player_entity)
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
        .add_system(Stage::PreRender, move |world, _resources, _time| {
            let (state, mut _animation) = world
                .query_one_mut::<(&PlayerState, &mut AnimatedSprite)>(player_entity)
                .unwrap();

            if state.previous != PlayerStateEnum::Flying && state.current == PlayerStateEnum::Flying
            {
                _animation.frames = vec!["ship_1".into(), "ship_2".into()];
                _animation.start_time = 0.0;
                _animation.fps = 4.0;
            } else if state.previous != PlayerStateEnum::Idle
                && state.current == PlayerStateEnum::Idle
            {
                _animation.frames = vec!["ship_0".into()];
                _animation.start_time = 0.0;
                _animation.fps = 1.0;
            }
        });

    engine
        .schedule_mut()
        .add_system(Stage::PreRender, |world, resources, _time| {
            let mut player_pos = Vec2::ZERO;
            for (_player, pos) in world.query_mut::<(&Player, &Position)>() {
                player_pos = pos.0;
            }
            let camera = resources.get_mut::<Camera>().unwrap();
            camera.position = player_pos - camera.viewport * 0.5;
        });

    engine
        .schedule_mut()
        .add_system(Stage::UIUpdate, |_world, resources, time| {
            if let Some(fps_debug) = resources.get_mut::<FpsDebug>() {
                fps_debug.elapsed += time.delta;
                fps_debug.frames += 1;

                if fps_debug.elapsed >= 1.0 {
                    println!("{} fps", fps_debug.frames);
                    fps_debug.elapsed = 0.0;
                    fps_debug.frames = 0;
                }
            }
        });

    engine.run();
}
