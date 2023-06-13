use std::{ptr::null_mut, iter::FlatMap, ops::Neg};
#[allow(dead_code)]

use std::{vec::Vec, ops::Index, clone::Clone, marker::Copy, collections::btree_map::OccupiedEntry};
use rand::*;
use crate::world::*;

const MAGIC_GENE_DECISION_WORD: u16 = 0x4C65;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Compass {
    North = 0,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputNeurons {
    // Spacial information. 
    FoodLeftRight = 0,
    FoodUpDown,
    FoodForward,
    FoodDensity,

    PheromoneLeftRight,
    PheromoneUpDown,
    PheromoneForward,
    PheromoneDensity,

    BlockageLeftRight,
    BlockageUpDown,
    BlockageForward,

    PopLeftRight,
    PopUpDown,
    PopForward,
    PopDensity,

    LocationX,
    LocationY,

    // History
    Age,

    KillCount, // Disallowed when kills are disabled
    
    LastMoveX,
    LastMoveY,

    // Racism
    GeneticSimilarity,
    
    // misc
    Random,
    Oscilator,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InternalNeurons {
    // Hyperbolic trig
    Tanh = 0,
    Cosh,
    Sinh,

    // Typical Ops
    Abs,
    Neg,
    Avg,
    Sqrt,
    InverseSqrt,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputNeurons {
    // Misc
    SetOscilator,
    EmitPheromone,
    SetResponsiveness,
    
    // Movement
    Move,
    MoveX,
    MoveY,
    MoveRandom,

    // Violence
    KillFoward,
}

impl OutputNeurons {
    pub fn from_int(integer: i32) -> Self {
        match integer % 8 {
            0 => Self::SetOscilator,
            1 => Self::EmitPheromone,
            2 => Self::SetResponsiveness,
            3 => Self::Move,
            4 => Self::MoveX,
            5 => Self::MoveY,
            6 => Self::MoveRandom,
            7 => Self::KillFoward,
            _ => Self::KillFoward,
        }
    }
}

impl InternalNeurons {
    pub fn from_int(integer: i32) -> Self {
        match integer % 8 {
            0 => Self::Tanh,
            1 => Self::Cosh,
            2 => Self::Sinh,
            3 => Self::Abs,
            4 => Self::Neg,
            5 => Self::Avg,
            6 => Self::Sqrt,
            7 => Self::InverseSqrt,
            _ => Self::Tanh,
        }
    }

    pub fn handle(&self, inputs: &Vec<GeneInput>, all_inputs: &Vec<Vec<GeneInput>>) -> f64 {
        let mut sum = 0.0;
        let mut ctr = 0;

        for i in inputs.as_slice() {
            sum += match *i {
                GeneInput::Input(f) => {
                    if f.is_nan() == false && f.is_finite() {
                        f
                    } else {
                        0
                    }
                },
                GeneInput::Internal(neuron, cell_index) => {
                    if neuron == self {
                        0.0
                    } else {
                        neuron.handle(all_inputs.index(cell_index), all_inputs)
                    }
                },
                GeneInput::Empty => {
                    0.0
                },
            };

            ctr += 1;
        }

        match *self {
            Self::Tanh => sum.tanh(),
            Self::Cosh => sum.cosh(),
            Self::Sinh => sum.sinh(),
            Self::Abs => sum.abs().tanh(),
            Self::Neg => sum.neg().tanh(),
            Self::Avg => sum/(ctr as f64),
            Self::Sqrt => sum.sqrt(),
            Self::InverseSqrt => 1.0/sum.sqrt(),
        }
    }
}

impl InputNeurons {
    pub fn from_int(integer: i32) -> Self {
        match integer % 24 {
            0 => Self::FoodLeftRight,
            1 => Self::FoodUpDown,
            2 => Self::FoodForward,
            3 => Self::FoodDensity,
            4 => Self::PheromoneLeftRight,
            5 => Self::PheromoneUpDown,
            6 => Self::PheromoneForward,
            7 => Self::PheromoneDensity,
            8 => Self::BlockageLeftRight,
            9 => Self::BlockageUpDown,
            10 => Self::BlockageForward,
            11 => Self::PopLeftRight,
            12 => Self::PopUpDown,
            13 => Self::PopForward,
            14 => Self::PopDensity,
            15 => Self::LocationX,
            16 => Self::LocationY,
            17 => Self::Age,
            18 => Self::KillCount,
            19 => Self::LastMoveX,
            20 => Self::LastMoveY,
            21 => Self::GeneticSimilarity,
            22 => Self::Random,
            23 => Self::Oscilator,
            _ => Self::Random
        }
    }

    pub fn handle(&mut self, cell: &Cell, grid: &Grid) -> f64 {
        match *self {
            Self::FoodLeftRight => {
                if grid[Position::new(cell.position.x + 1, cell.position.y)].has_food {
                    1.0
                } else if grid[Position::new(cell.position.x - 1, cell.position.y)].has_food {
                    -1.0
                } else {
                    0.0
                }
            },
            Self::FoodUpDown => {
                if grid[Position::new(cell.position.x, cell.position.y + 1)].has_food {
                    1.0
                } else if grid[Position::new(cell.position.x, cell.position.y - 1)].has_food {
                    -1.0
                } else {
                    0.0
                }
            },
            Self::FoodForward => {
                match cell.rotation {
                    Compass::North => {
                        if grid[Position::new(cell.position.x, cell.position.y + 1)].has_food {
                            1.0
                        } else if grid[Position::new(cell.position.x, cell.position.y - 1)].has_food {
                            -1.0
                        } else {
                            0.0
                        }
                    },
                    Compass::South => {
                        if grid[Position::new(cell.position.x, cell.position.y - 1)].has_food {
                            1.0
                        } else if grid[Position::new(cell.position.x, cell.position.y + 1)].has_food {
                            -1.0
                        } else {
                            0.0
                        }
                    },
                    Compass::East => {
                        if grid[Position::new(cell.position.x + 1, cell.position.y)].has_food {
                            1.0
                        } else if grid[Position::new(cell.position.x - 1, cell.position.y)].has_food {
                            -1.0
                        } else {
                            0.0
                        }
                    },
                    Compass::West => {
                        if grid[Position::new(cell.position.x - 1, cell.position.y)].has_food {
                            1.0
                        } else if grid[Position::new(cell.position.x + 1, cell.position.y)].has_food {
                            -1.0
                        } else {
                            0.0
                        }
                    },
                }
            },
            Self::FoodDensity => {
                let mut count = 0.0;
                let pos = cell.position;
                count += if grid[pos].has_food {
                    1.0
                } else {
                    0.0
                };
                count += if grid[Position::new(pos.x, pos.y + 1)].has_food {
                    1.0
                } else {
                    0.0
                };
                count += if grid[Position::new(pos.x + 1, pos.y + 1)].has_food {
                    1.0
                } else {
                    0.0
                };
                count += if grid[Position::new(pos.x + 1, pos.y)].has_food {
                    1.0
                } else {
                    0.0
                };
                count += if grid[Position::new(pos.x + 1, pos.y - 1)].has_food {
                    1.0
                } else {
                    0.0
                };
                count += if grid[Position::new(pos.x, pos.y - 1)].has_food {
                    1.0
                } else {
                    0.0
                };
                count += if grid[Position::new(pos.x - 1, pos.y - 1)].has_food {
                    1.0
                } else {
                    0.0
                };
                count += if grid[Position::new(pos.x - 1, pos.y)].has_food {
                    1.0
                } else {
                    0.0
                };
                count += if grid[Position::new(pos.x - 1, pos.y + 1)].has_food {
                    1.0
                } else {
                    0.0
                };
                count/9.0
            },
            Self::PheromoneLeftRight => {
                if grid[Position::new(cell.position.x + 1, cell.position.y)].pheromone_level != 0.0 {
                    grid[Position::new(cell.position.x + 1, cell.position.y)].pheromone_level/50.0
                } else if grid[Position::new(cell.position.x - 1, cell.position.y)].pheromone_level != 0.0 {
                    grid[Position::new(cell.position.x - 1, cell.position.y)].pheromone_level/-50.0
                } else {
                    0.0
                }
            },
            Self::PheromoneUpDown => {
                if grid[Position::new(cell.position.x, cell.position.y + 1)].pheromone_level != 0.0 {
                    grid[Position::new(cell.position.x, cell.position.y + 1)].pheromone_level/50.0
                } else if grid[Position::new(cell.position.x, cell.position.y - 1)].pheromone_level != 0.0 {
                    grid[Position::new(cell.position.x, cell.position.y - 1)].pheromone_level/-50.0
                } else {
                    0.0
                }
            },
            Self::PheromoneForward => {
                match cell.rotation {
                    Compass::North => {
                        if grid[Position::new(cell.position.x, cell.position.y + 1)].pheromone_level != 0.0 {
                            grid[Position::new(cell.position.x, cell.position.y + 1)].pheromone_level/50.0
                        } else if grid[Position::new(cell.position.x, cell.position.y - 1)].pheromone_level != 0.0 {
                            grid[Position::new(cell.position.x, cell.position.y - 1)].pheromone_level/-50.0
                        } else {
                            0.0
                        }
                    },
                    Compass::South => {
                        if grid[Position::new(cell.position.x, cell.position.y - 1)].pheromone_level != 0.0 {
                            grid[Position::new(cell.position.x, cell.position.y - 1)].pheromone_level/50.0
                        } else if grid[Position::new(cell.position.x, cell.position.y + 1)].pheromone_level != 0.0 {
                            grid[Position::new(cell.position.x, cell.position.y + 1)].pheromone_level/-50.0
                        } else {
                            0.0
                        }
                    },
                    Compass::East => {
                        if grid[Position::new(cell.position.x + 1, cell.position.y)].pheromone_level != 0.0 {
                            grid[Position::new(cell.position.x + 1, cell.position.y)].pheromone_level/50.0
                        } else if grid[Position::new(cell.position.x - 1, cell.position.y)].pheromone_level != 0.0 {
                            grid[Position::new(cell.position.x - 1, cell.position.y)].pheromone_level/-50.0
                        } else {
                            0.0
                        }
                    },
                    Compass::West => {
                        if grid[Position::new(cell.position.x - 1, cell.position.y)].pheromone_level != 0.0 {
                            grid[Position::new(cell.position.x - 1, cell.position.y)].pheromone_level/50.0
                        } else if grid[Position::new(cell.position.x + 1, cell.position.y)].pheromone_level != 0.0 {
                            grid[Position::new(cell.position.x + 1, cell.position.y)].pheromone_level/-50.0
                        } else {
                            0.0
                        }
                    },
                }
            },
            Self::PheromoneDensity => {
                let mut pheromones = 0.0;
                let pos = cell.position;
                pheromones += grid[Position::new(pos.x, pos.y)].pheromone_level;
                pheromones += grid[Position::new(pos.x, pos.y + 1)].pheromone_level;
                pheromones += grid[Position::new(pos.x + 1, pos.y + 1)].pheromone_level;
                pheromones += grid[Position::new(pos.x + 1, pos.y)].pheromone_level;
                pheromones += grid[Position::new(pos.x + 1, pos.y - 1)].pheromone_level;
                pheromones += grid[Position::new(pos.x, pos.y - 1)].pheromone_level;
                pheromones += grid[Position::new(pos.x - 1, pos.y + 1)].pheromone_level;
                pheromones += grid[Position::new(pos.x - 1, pos.y)].pheromone_level;
                pheromones += grid[Position::new(pos.x - 1, pos.y - 1)].pheromone_level;
                pheromones/450.0
            },
            Self::BlockageLeftRight | Self::PopLeftRight => {
                if grid[Position::new(cell.position.x + 1, cell.position.y)].cell != null_mut() {
                    1.0
                } else if grid[Position::new(cell.position.x - 1, cell.position.y)].cell != null_mut() {
                    -1.0
                } else {
                    0.0
                }
            },
            Self::BlockageUpDown | Self::PopUpDown => {
                if grid[Position::new(cell.position.x, cell.position.y + 1)].cell != null_mut() {
                    1.0
                } else if grid[Position::new(cell.position.x, cell.position.y - 1)].cell != null_mut() {
                    -1.0
                } else {
                    0.0
                }
            },
            Self::BlockageForward | Self::PopForward => {
                match cell.rotation {
                    Compass::North => {
                        if grid[Position::new(cell.position.x, cell.position.y + 1)].cell != null_mut() {
                            1.0
                        } else if grid[Position::new(cell.position.x, cell.position.y - 1)].cell != null_mut() {
                            -1.0
                        } else {
                            0.0
                        }
                    },
                    Compass::South => {
                        if grid[Position::new(cell.position.x, cell.position.y - 1)].cell != null_mut() {
                            1.0
                        } else if grid[Position::new(cell.position.x, cell.position.y + 1)].cell != null_mut() {
                            -1.0
                        } else {
                            0.0
                        }
                    },
                    Compass::East => {
                        if grid[Position::new(cell.position.x + 1, cell.position.y)].cell != null_mut() {
                            1.0
                        } else if grid[Position::new(cell.position.x - 1, cell.position.y)].cell != null_mut() {
                            -1.0
                        } else {
                            0.0
                        }
                    },
                    Compass::West => {
                        if grid[Position::new(cell.position.x - 1, cell.position.y)].cell != null_mut() {
                            1.0
                        } else if grid[Position::new(cell.position.x + 1, cell.position.y)].cell != null_mut() {
                            -1.0
                        } else {
                            0.0
                        }
                    },
                }
            },
            Self::PopDensity => {
                let mut count = 1.0;
                let pos = cell.position;
                count += if grid[Position::new(pos.x, pos.y + 1)].cell != null_mut() {
                    1.0
                } else {
                    0.0
                };
                count += if grid[Position::new(pos.x + 1, pos.y + 1)].cell != null_mut() {
                    1.0
                } else {
                    0.0
                };
                count += if grid[Position::new(pos.x + 1, pos.y)].cell != null_mut() {
                    1.0
                } else {
                    0.0
                };
                count += if grid[Position::new(pos.x + 1, pos.y - 1)].cell != null_mut() {
                    1.0
                } else {
                    0.0
                };
                count += if grid[Position::new(pos.x, pos.y - 1)].cell != null_mut() {
                    1.0
                } else {
                    0.0
                };
                count += if grid[Position::new(pos.x - 1, pos.y - 1)].cell != null_mut() {
                    1.0
                } else {
                    0.0
                };
                count += if grid[Position::new(pos.x - 1, pos.y)].cell != null_mut() {
                    1.0
                } else {
                    0.0
                };
                count += if grid[Position::new(pos.x - 1, pos.y + 1)].cell != null_mut() {
                    1.0
                } else {
                    0.0
                };
                count/9.0
            },
            Self::LocationX => {
                ((cell.position.x as f64) * (2.0)/((grid.get_x() - 1) as f64)) - 1.0
            },
            Self::LocationY => {
                ((cell.position.y as f64) * (2.0)/((grid.get_y() - 1) as f64)) - 1.0
            },
            Self::Age => {
                // TODO: this
                0.0
            },
            Self::KillCount => {
                // TODO: this
                0.0
            },
            Self::LastMoveX => {
                cell.last_move.x as f64
            },
            Self::LastMoveY => {
                cell.last_move.y as f64
            },
            Self::GeneticSimilarity => {
                // TODO: this
                0.0
            },
            Self::Random => {
                thread_rng().gen_range(-1.0..=1.0)
            },
            Self::Oscilator => {
                cell.oscilator.get_state()
            },
        }
    }
}

type Gene = i32;

pub fn encode_gene(input: i32, output: i32, weight: u16, input_is_internal: bool, output_is_internal: bool) -> Gene {
    let mut ret = ((input & 0x7f) << 24) | ((output & 0x7f) << 16);
    ret |= if input_is_internal {
        1 << 31
    } else {
        0
    };

    ret |= if output_is_internal {
        1 << 23
    } else {
        0
    };

    ret | (weight as i32)
}

pub fn decode_gene(gene: Gene) -> (i32, i32, u16, bool, bool) {
    let input_is_internal = (gene >> 31) == 1;
    let output_is_internal = ((gene >> 23) & 1) == 1;
    let input: i32 = (gene >> 24) & 0x7f;
    let output: i32 = (gene >> 16) & 0x7f;
    let weight: u16 = ((gene) & 0xffff) as u16;

    (input, output, weight, input_is_internal, output_is_internal)
}


#[derive(Debug,Clone, Copy)]
pub struct Oscilator {
    counter: f64,
    frequency: f64,
    state: bool,
}

impl Oscilator {
    pub fn get_state(&self) -> f64 {
        if self.state {
            1.0
        } else {
            -1.0
        }
    }

    pub fn update(&mut self) -> () {
        self.counter += self.frequency;
        if self.counter > 1.0 {
            self.counter = 0.0;
            self.state = !self.state;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub genes: Vec<Gene>,
    pub position: Position<usize>,
    pub last_move: Position<usize>,
    pub food_level: u32,
    pub rotation: Compass,
    pub oscilator: Oscilator,
}

impl Index<usize> for Cell {
    type Output = Gene;

    fn index(&self, index: usize) -> &Self::Output {
        self.genes.index(index)
    }
}

impl Cell {
    pub fn create_cell(gene_count: usize) -> Cell {
        Cell {
            genes: vec![0; gene_count],
            position: Position { x: 0, y: 0 },
            last_move: Position { x: 0, y: 0 },
            food_level: 10,
            rotation: match rand::thread_rng().gen::<u8>() % 4 { 0 => Compass::North, 1 => Compass::South, 2 => Compass::East, 3 => Compass::West, _ => Compass::East },
            oscilator: Oscilator { counter: 0.0, frequency: 0.1, state: false }
        }
    }

    pub fn generate_offspring(&self) -> Cell {
        let mut ret = self.clone();
        let len = ret.genes.len();
        
        if MAGIC_GENE_DECISION_WORD == rand::thread_rng().gen::<u16>() {
            ret.genes[rand::thread_rng().gen::<usize>() % len] ^= 1 << (rand::thread_rng().gen::<u8>() & 0x1f);
        }

        ret.position = self.position;
        ret.last_move = Position::new(0, 0);
        ret
    }
}
