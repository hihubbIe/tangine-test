use glam::Vec2;
use tangine::{
    api::Engine,
    ecs::{AnimatedSprite, Friction, MaxSpeed, Player, Position, StaticSprite, Tint, Velocity},
    input::{InputState, KeyCode},
    render::camera::Camera,
    schedule::Stage,
};

fn main() {
    let mut engine = Engine::new();

    engine.world_mut().spawn((
        Player,
        Position(Vec2::new(100.0, 100.0)),
        Velocity(Vec2::ZERO),
        Friction(0.97),
        MaxSpeed(1000.0),
        AnimatedSprite {
            frames: vec!["ship_0".into(), "ship_1".into(), "ship_2".into()],
            fps: 2.0,
            looping: true,
            start_time: 0.0,
        },
    ));

    engine.world_mut().spawn((
        Position(Vec2::new(300.0, 300.0)),
        StaticSprite {
            region: "ship_0".into(),
        },
        Tint {
            r: 0.3,
            ..Default::default()
        },
    ));

    engine
        .schedule_mut()
        .add_system(Stage::PrePhysics, |world, resources, time| {
            let input = resources.get::<InputState>().unwrap();
            let vel_increase = 800.0 * time.delta;
            for (_player, vel) in world.query_mut::<(&Player, &mut Velocity)>() {
                if input.is_pressed(KeyCode::KeyW) {
                    vel.0.y -= vel_increase;
                }
                if input.is_pressed(KeyCode::KeyS) {
                    vel.0.y += vel_increase;
                }
                if input.is_pressed(KeyCode::KeyA) {
                    vel.0.x -= vel_increase;
                }
                if input.is_pressed(KeyCode::KeyD) {
                    vel.0.x += vel_increase;
                }
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

    engine.run();
}
