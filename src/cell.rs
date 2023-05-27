use std::{vec::Vec, ops::Index, clone::Clone, marker::Copy};
use rand::rngs::ThreadRng;
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
pub enum ImputNeurons {
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

#[derive(Debug, Clone, Copy)]
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

type Gene = i32;

pub fn encode_gene(input: i32, output: i32, weight: u16, input_is_internal: bool, output_is_internal: bool) -> Gene {
    let ret = ((input & 0x7f) << 24) | ((output & 0x7f) << 16);
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

    ret | weight
}

pub fn decode_gene(gene: Gene) -> (i32, i32, u16, bool, bool) {
    let input_is_internal = ((gene >> 31) == 1);
    let output_is_internal = (((gene >> 23) & 1) == 1);
    let input: i32 = (gene >> 24) & 0x7f;
    let output: i32 = (gene >> 16) & 0x7f;
    let weight: u16 = ((gene) & 0xffff) as u16;

    (input, output, weight, input_is_internal, output_is_internal)
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub(in crate) genes: Vec<Gene>,
    pub(in crate) position: Position<usize>,
    pub(in crate) food_level: u32,
}

impl Index<usize> for Cell {
    type Output = Gene;

    fn index(&self, index: usize) -> &Self::Output {
        genes[index]
    }
}

pub impl Cell {
    pub fn generate_offspring(&self, ) -> Cell {
        let ret: Cell;
        ret.genes = self.clone();
        
        if MAGIC_GENE_DECISION_WORD == thread_rng().gen::<i16>()
        {
            ret.genes[thread_rng().gen::<usize>() % ret.genes.len()] ^= 1 << (thread_rng().gen::<8>() & 0x1f);
        }

        ret.position = self.position;
        ret
    }
}