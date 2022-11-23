mod components;
mod systems;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*, utils::HashMap,
};
use components::{State, Config, RevivedEvent, StarvedEvent, Theme, Cells, Position};
use systems::*;
use rand::Rng;

fn main() {
    App::new()
        .insert_resource(Config {
            width: 300,
            height: 300,
            board_color: Color::rgb(0.2, 0.2, 0.2),
            alive_color: Color::rgb(0.8, 0.8, 0.8),
            dead_color: Color::rgb(0.1, 0.1, 0.1),
        })
        .add_event::<RevivedEvent>()
        .add_event::<StarvedEvent>()
        .add_startup_system(setup)
        .add_system(tick)
        .add_system(revive.after(tick))
        .add_system(starve.after(tick))
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 800.0,
                height: 800.0,
                ..default()
            },
            ..default()
        }))
        .run();
}

fn setup(mut commands: Commands, windows: Res<Windows>, config: Res<Config>) {
    let theme = Theme {
        board_color: config.board_color,
        alive_color: config.alive_color,
        dead_color: config.dead_color,
    };

    let mut cells = Cells(HashMap::default());

    let screen_height = windows.get_primary().unwrap().height();
    let cell_size = Vec2::splat(screen_height / config.height as f32);
    let offset = cell_size.extend(0.0) / 2.0;

    let center_cell = Vec2::new(config.width as f32, config.height as f32) / 2.0 * cell_size;

    commands
        .spawn(Camera2dBundle::default())
        .insert(Transform::from_translation(center_cell.extend(999.9)));

    commands.spawn(SpriteBundle {
        transform: Transform::from_translation(center_cell.extend(0.0)),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(screen_height) + Vec2::splat(2.0)),
            color: theme.board_color,
            ..Default::default()
        },
        ..Default::default()
    });

    let mut rng = rand::thread_rng();

    for y in 0..config.height {
        for x in 0..config.width {
            let cell = IVec2::new(x, y);
            let translation = cell_size.extend(0.0) * cell.extend(0).as_vec3() + offset;
            let transform = Transform::from_translation(translation);

            let state;
            let color;

            if rng.gen_bool(0.5) {
                state = State::Alive;
                color = theme.alive_color;
            } else {
                state = State::Dead;
                color = theme.dead_color;
            }
            let entity = commands
                .spawn(SpriteBundle {
                    transform,
                    sprite: Sprite {
                        custom_size: Some(cell_size - Vec2::splat(0.0)),
                        color,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(state)
                .insert(Position(cell))
                .id();

            cells.0.insert(Position(cell), entity);
        }
    }

    commands.insert_resource(theme);
    commands.insert_resource(cells);
}