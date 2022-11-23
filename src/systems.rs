use bevy::prelude::*;
use crate::components::{State, Theme, StarvedEvent, RevivedEvent, Cells};

pub fn tick(
    mut starved_writer: EventWriter<StarvedEvent>,
    mut revived_writer: EventWriter<RevivedEvent>,
    cells: Res<Cells>,
    query: Query<&State>,
) {
    for (coords, entity) in cells.0.clone().iter() {
        let mut alive_count = 0;

        for n_coords in coords.get_neighbors().iter() {
            if let Some(n_entity) = cells.0.get(n_coords) {
                if let Ok(state) = query.get(*n_entity) {
                    if *state == State::Alive {
                        alive_count += 1;
                    }
                }
            }
        }

        match query.get(*entity).unwrap() {
            State::Alive => {
                if alive_count > 3 || alive_count < 2 {
                    starved_writer.send(StarvedEvent { entity: *entity });
                }
            }
            State::Dead => {
                if alive_count == 3 {
                    revived_writer.send(RevivedEvent { entity: *entity });
                }
            }
        }
    }
}

pub fn revive(
    theme: Res<Theme>,
    mut reader: EventReader<RevivedEvent>,
    mut query: Query<(&mut State, &mut Sprite)>,
) {
    for event in reader.iter() {
        if let Ok((mut state, mut sprite)) = query.get_mut(event.entity) {
            *state = State::Alive;
            sprite.color = theme.alive_color;
        }
    }
}

pub fn starve(
    theme: Res<Theme>,
    mut reader: EventReader<StarvedEvent>,
    mut query: Query<(&mut State, &mut Sprite)>,
) {
    for event in reader.iter() {
        if let Ok((mut state, mut sprite)) = query.get_mut(event.entity) {
            *state = State::Dead;
            sprite.color = theme.dead_color;
        }
    }
}