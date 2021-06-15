//! For when a regular `Wrapper<T>(T)` gets too boring.

use rand::{thread_rng, Rng};
use std::{error::Error, fmt};

/// The wrapper.
#[derive(Debug, Clone, PartialEq)]
pub struct ChanceTheWrapper<T> {
    chance: f64,
    value: T,
}

impl<T> ChanceTheWrapper<T> {
    /// Creates a new wrapper with a 50% chance of succeeding.
    pub fn new(value: T) -> Self {
        Self { chance: 0.5, value }
    }

    /// Creates a new wrapper.
    ///
    /// The given chance must not be less than `0.0`, greater than `1.0`, or NaN.
    pub fn with_chance<C: Into<f64>>(value: T, chance: C) -> Result<Self, InvalidChance> {
        let chance = chance.into();
        if chance < 0.0 || chance > 1.0 || chance.is_nan() {
            return Err(InvalidChance);
        }
        Ok(Self { chance, value })
    }

    fn maybe(&self) -> bool {
        thread_rng().gen_bool(self.chance)
    }

    /// Maybe returns a reference to the value.
    pub fn get(&self) -> Option<&T> {
        self.maybe().then(|| &self.value)
    }

    /// Maybe returns a mutable reference to the value.
    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.maybe().then(move || &mut self.value)
    }

    /// Maybe turns the wrapper into the value.
    pub fn into_value(self) -> Option<T> {
        self.maybe().then(move || self.value)
    }
}

/// Error for a invalid chance.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct InvalidChance;

impl Error for InvalidChance {}

impl fmt::Display for InvalidChance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("chance must be between 0.0 and 1.0")
    }
}
