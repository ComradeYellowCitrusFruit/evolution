#[allow(dead_code)]

use crate::cell::*;
use std::{vec::Vec, ops::Index, ptr::null_mut};
use rand::*;

#[derive(Debug, Clone, Copy)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}

impl<T> Position<T> {
    pub fn new(x: T, y: T) -> Position<T> {
        Position { x: x, y: y }
    }
}
#[derive(Debug, Clone)]
pub struct Grid {
    x: usize,
    y: usize,
    internal: Vec<Tile>,
}

impl Grid {
    fn init(xp: usize, yp: usize) -> Grid {
        Grid {
            x: xp,
            y: yp,
            internal: vec![Tile { has_food: false, pheromone_level: 0.0, cell: null_mut() }; xp * yp],
        }
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
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
    pub has_food: bool,
    pub pheromone_level: f64,
    pub cell: *mut Cell, // ik you're not supposed to use pointers in Rust but whatever.
}

pub struct World {
    cell_list: Vec<Cell>,
    grid: Grid,
}

#[derive(Clone, PartialEq)]
pub enum GeneInput {
    Input(f64),
    Internal(InternalNeurons, usize),
    Empty,
}

impl World {
    pub fn new_world(population: usize, gene_count: usize, x: usize, y: usize) -> World {
        let mut ret = World {
            cell_list: vec![Cell::create_cell(gene_count); population],
            grid: Grid::init(x, y),
        };

        for cell in ret.cell_list.as_mut_slice() { // I hate Rust mutability rules
            loop { // Why tf doesnt Rust have do { ... } while?
                cell.position.x = thread_rng().gen::<usize>() % ret.grid.x;
                cell.position.y = thread_rng().gen::<usize>() % ret.grid.y;
                if ret.grid[cell.position].cell != null_mut() { break; }
            }
        }

        ret
    }
    
    pub fn step(&self) -> () {
        let mut gene_inputs: Vec<Vec<GeneInput>> = Vec::with_capacity(self.cell_list.len());
        for i in 0..self.cell_list.len() {
            let cell = self.cell_list.index(i);
            gene_inputs.push(vec![GeneInput::Empty; 15]);
            let inputs = gene_inputs.last_mut().unwrap();

            for j in cell.genes.as_slice() {
                let unpacked = decode_gene(*j);
                let index = if unpacked.4 {
                    (unpacked.1 % 8) + 8 
                } else {
                    unpacked.1 % 8
                } as usize;
                
                let input = if unpacked.3 {
                    GeneInput::Internal(InternalNeurons::from_int(unpacked.0), i)
                } else {
                    GeneInput::Input(InputNeurons::from_int(unpacked.0).handle(cell, &self.grid))
                };
                inputs[index] = input; // TODO: Multiple inputs
            }
        }

        for i in 0..self.cell_list.len() {
            let cell = self.cell_list.index(i);
            let inputs = gene_inputs.index(i);

            for j in inputs.as_slice() {
                if *j == GeneInput::Empty {
                    continue;
                }


            }
        }
    }
}
