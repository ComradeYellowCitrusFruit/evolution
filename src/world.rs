use crate::cell::*;
use std::{vec::Vec, option::Option, boxed::Box, ops::Index};
use rand::rngs::ThreadRng;

#[derive(Debug, Clone, Copy)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy)]
struct Grid<T> {
    x: usize,
    y: usize,
    internal: Vec<T>,
}

impl<T> Grid<T> {
    fn init<T>(xp: usize, yp: usize) -> Grid<T> {
        let ret: Grid<T> = Grid<T> {
            x: xp,
            y: yp,
            internal: Vec::with_capacity(xp * yp),
        };

        ret
    }
}

impl<T> Index<Position<usize>> for Grid<T> {
    type Output = Option<&T>;

    fn index(&self, index: Position<usize>) -> Self::Output {
        let ret = if index.y > y || index.x > x {
            None
        } else {
            Some(genes[x * index.y + index.x])
        };

        ret
    }
}

pub struct Tile {
    pub(self) food: bool,
    pub(self) pheromone_level: f32,
    pub(self) cell: Option<&mut Cell>, // In C/C++ we'd use a pointer lmao
}

pub struct World {
    cell_list: Vec<Cell>,
    grid: Grid<Tile>,
}

// TODO: most of the code will probably go here