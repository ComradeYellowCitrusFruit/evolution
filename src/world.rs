use crate::cell::*;
use std::{vec::Vec, option::Option, ops::Index};
use rand::*;

#[derive(Debug, Clone, Copy)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone)]
struct Grid<T> {
    x: usize,
    y: usize,
    internal: Vec<T>,
}

impl<T> Grid<T> {
    fn init(xp: usize, yp: usize) -> Grid<T> {
        let ret: Grid<T> = Grid {
            x: xp,
            y: yp,
            internal: Vec::with_capacity(xp * yp),
        };

        ret
    }
}

impl<T> Index<Position<usize>> for Grid<T> {
    type Output = T;

    fn index(&self, index: Position<usize>) -> &Self::Output {
        let ret = if index.y >= self.y || index.x >= self.x {
            self.internal.index(self.x * (self.y - 1) + (self.x - 1))
        } else {
            self.internal.index(self.x * index.y + index.x)
        };

        ret
    }
}

pub struct Tile {
    pub(self) food: bool,
    pub(self) pheromone_level: f32,
    pub(self) cell: Option<Cell>, // In C/C++ we'd use a pointer lmao
}

pub struct World {
    cell_list: Vec<Cell>,
    grid: Grid<Tile>,
}

// TODO: most of the code will probably go here