use std::{num::ParseIntError, str::FromStr};

pub struct Food {
    pub calories: u32,
}

impl Food {
    pub fn new(calories: u32) -> Self {
        Self { calories }
    }
}

impl FromStr for Food {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.parse::<u32>()?))
    }
}
