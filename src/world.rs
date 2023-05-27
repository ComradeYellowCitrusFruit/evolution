use crate::cell::{*, self};
use std::{vec::Vec, option::Option, ops::Index};
use rand::*;

#[derive(Debug, Clone, Copy)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone)]
struct Grid {
    x: usize,
    y: usize,
    internal: Vec<Tile>,
}

impl Grid {
    fn init(xp: usize, yp: usize) -> Grid {
        Grid {
            x: xp,
            y: yp,
            internal: vec![Tile { has_food: false, pheromone_level: 0.0, cell: 0 as *mut Cell }; xp * yp],
        }
    }
}

impl Index<Position<usize>> for Grid {
    type Output = Tile;

    fn index(&self, index: Position<usize>) -> &Self::Output {
        let ret = if index.y > self.y || index.x >= self.x {
            // Make sure we select the correct overflow response
            if index.y > self.y && index.x < self.x { 
                self.internal.index(self.x * self.y + index.x)
            } else if index.y < self.y && index.x >= self.x {
                self.internal.index(self.x * index.y + (self.x - 1))
            } else {
                self.internal.index(self.x * self.y + (self.x - 1))
            }
        } else {
            self.internal.index(self.x * index.y + index.x)
        };

        ret
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub(self) has_food: bool,
    pub(self) pheromone_level: f32,
    pub(self) cell: *mut Cell, // ik you're not supposed to use pointers in Rust but whatever.
}

pub struct World {
    cell_list: Vec<Cell>,
    grid: Grid,
}

impl World {
    pub fn new_world(population: usize, gene_count: usize, x: usize, y: usize) -> World {
        World {
            cell_list: vec![Cell::create_cell(gene_count); population],
            grid: Grid::init(x, y),
        }
    }
}

// TODO: most of the code will probably go here