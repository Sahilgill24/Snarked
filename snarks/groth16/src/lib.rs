// Simplified Groth16 SNARK implementation

pub mod interfaces;
pub mod preprocessing;
pub mod primitives;
pub mod protocol;
pub mod trusted_setup;
pub mod utils;

#[cfg(test)]
pub mod tests;
