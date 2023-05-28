use std::{vec::Vec, ops::Index, clone::Clone, marker::Copy};
use rand::*;
use crate::world::Position;

const MAGIC_GENE_DECISION_WORD: u16 = 0x4C65;

#[derive(Debug, Clone, Copy)]
pub enum Compass {
    North = 0,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
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

impl InputNeurons {
    pub fn from_int(int: u8) -> InputNeurons {
        match int % ((InputNeurons::Oscilator as u8) + 1) {
            0 => InputNeurons::FoodLeftRight,
            1 => InputNeurons::FoodUpDown,
            2 => InputNeurons::FoodForward,
            3 => InputNeurons::FoodDensity,
            4 => InputNeurons::PheromoneLeftRight,
            5 => InputNeurons::PheromoneUpDown,
            6 => InputNeurons::PheromoneForward,
            7 => InputNeurons::PheromoneDensity,
            8 => InputNeurons::BlockageLeftRight,
            9 => InputNeurons::BlockageUpDown,
            10 => InputNeurons::BlockageForward,
            11 => InputNeurons::PopLeftRight,
            12 => InputNeurons::PopUpDown,
            13 => InputNeurons::PopForward,
            14 => InputNeurons::PopDensity,
            15 => InputNeurons::LocationX,
            16 => InputNeurons::LocationY,
            17 => InputNeurons::Age,
            18 => InputNeurons::KillCount,
            19 => InputNeurons::LastMoveX,
            20 => InputNeurons::LastMoveY,
            21 => InputNeurons::GeneticSimilarity,
            22 => InputNeurons::Random,
            23 => InputNeurons::Oscilator,
            _ => InputNeurons::Random,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InteralNeurons {
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

impl InteralNeurons {
    pub fn from_int(int: u8) -> InteralNeurons {
        match int % ((InteralNeurons::InverseSqrt as u8) + 1) {
            0 => InteralNeurons::Tanh,
            1 => InteralNeurons::Cosh,
            2 => InteralNeurons::Sinh,
            3 => InteralNeurons::Abs,
            4 => InteralNeurons::Neg,
            5 => InteralNeurons::Avg,
            6 => InteralNeurons::Sqrt,
            7 => InteralNeurons::InverseSqrt,
            _ => InteralNeurons::Avg
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OutputNeurons {
    // Misc
    SetOscilator = 0,
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
    pub fn from_int(int: u8) -> OutputNeurons {
        match int % ((OutputNeurons::KillFoward as u8) + 1) {
            0 => OutputNeurons::SetOscilator,
            1 => OutputNeurons::EmitPheromone,
            2 => OutputNeurons::SetResponsiveness,
            3 => OutputNeurons::Move,
            4 => OutputNeurons::MoveX,
            5 => OutputNeurons::MoveY,
            6 => OutputNeurons::MoveRandom,
            7 => OutputNeurons::KillFoward,
            _ => OutputNeurons::Move,
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
    let input_is_internal = ((gene >> 31) == 1);
    let output_is_internal = (((gene >> 23) & 1) == 1);
    let input: i32 = (gene >> 24) & 0x7f;
    let output: i32 = (gene >> 16) & 0x7f;
    let weight: u16 = ((gene) & 0xffff) as u16;

    (input, output, weight, input_is_internal, output_is_internal)
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub(in crate) genes: Vec<Gene>,
    pub(in crate) position: Position<usize>,
    pub(in crate) last_move: Position<usize>,
    pub(in crate) food_level: u32,
    pub(in crate) kill_count: u32,
    pub(in crate) period: f32,
    pub(in crate) rotation: Compass,
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
            kill_count: 0,
            period: 0.0273972602739726,
            rotation: match rand::thread_rng().gen::<u8>() % 4 { 0 => Compass::North, 1 => Compass::South, 2 => Compass::East, 3 => Compass::West, _ => Compass::East },
        }
    }

    pub fn generate_offspring(&self) -> Cell {
        let mut ret = self.clone();
        let len = ret.genes.len();
        
        if MAGIC_GENE_DECISION_WORD == rand::thread_rng().gen::<u16>()
        {
            ret.genes[rand::thread_rng().gen::<usize>() % len] ^= 1 << (rand::thread_rng().gen::<u8>() & 0x1f);
        }

        ret.position = self.position;
        ret
    }
}