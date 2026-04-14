//! Deterministic procedural-generation building blocks used by Harmonius PCG acceptance tests.
#![cfg_attr(not(test), warn(missing_docs))]

pub mod attributes;
pub mod biome;
pub mod chunk;
pub mod cosmic;
pub mod csp;
pub mod graph;
pub mod noise;
pub mod planet;
pub mod points;
pub mod poisson;
pub mod seed;
pub mod shape_grammar;
pub mod socket;
pub mod spline;
pub mod stamp;
pub mod stars;
pub mod thin;
pub mod vegetation;
pub mod wfc;

#[cfg(test)]
mod tc;
