use bevy::{
    prelude::{Color, Component, Entity, IVec2, Resource},
    utils::HashMap,
};

#[derive(Resource)]
pub struct Config {
    pub width: i32,
    pub height: i32,
    pub board_color: Color,
    pub alive_color: Color,
    pub dead_color: Color,
}

#[derive(Resource)]
pub struct Theme {
    pub board_color: Color,
    pub alive_color: Color,
    pub dead_color: Color,
}

#[derive(Resource, Clone)]
pub struct Board(pub HashMap<Position, Entity>);

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(pub IVec2);

impl Position {
    pub fn get_neighbors(&self) -> [Position; 8] {
        [
            Position(self.0 + IVec2::new(-1, -1)),
            Position(self.0 + IVec2::new(0, -1)),
            Position(self.0 + IVec2::new(1, -1)),
            Position(self.0 + IVec2::new(-1, 0)),
            Position(self.0 + IVec2::new(1, 0)),
            Position(self.0 + IVec2::new(-1, 1)),
            Position(self.0 + IVec2::new(0, 1)),
            Position(self.0 + IVec2::new(1, 1)),
        ]
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Alive,
    Dead,
}

//Events
pub struct StarvedEvent(pub Entity);

pub struct RevivedEvent(pub Entity);
