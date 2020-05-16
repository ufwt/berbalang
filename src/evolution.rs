use serde::Serialize;
use std::cmp::PartialOrd;

pub trait Epochal {
    /// The evolve function turns the crank once on the evolutionary
    /// process.
    fn evolve(self) -> Self;
}

/// Types that implement Hatchery act as executors, and are responsible
/// for developing a genotype into a phenotype.
pub trait Hatchery {
    type Genome;
    type Phenome;

    fn hatch(&self, genotype: &Self::Genome) -> Self::Phenome;
}

pub trait Score {
    type Params;
    type Fitness;

    fn score(&self, params: &Self::Params) -> Self::Fitness;
}

/// Implement partial order
pub trait Fitness: PartialOrd + Serialize {}

pub trait Genotype {
    fn crossover(&self, mate: &Self) -> Vec<Self>
    where
        Self: Sized;
    fn mutate(&mut self);
}
