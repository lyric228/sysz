use std::{iter::repeat_with, ops::RangeInclusive, sync::Arc};

use rand::{
    Rng,
    distr::{Alphanumeric, Uniform, uniform::SampleUniform},
    rng,
};

use crate::{Error, Result};

/// Generates a random value within the inclusive range [min, max].
pub fn random<T>(min: T, max: T) -> Result<T>
where
    T: PartialOrd + Copy + SampleUniform,
{
    let mut rng = rng();
    let distr = Uniform::new_inclusive(min, max)?;
    Ok(rng.sample(distr))
}

/// Generates a random boolean (50% chance).
#[inline]
pub fn random_bool() -> bool {
    rand::random()
}

/// Generates a random string of `length`. Uses `charset` if provided, otherwise alphanumeric.
pub fn random_string(length: usize, charset: Option<&str>) -> Result<String> {
    let mut rng = rng();

    if let Some(chars) = charset {
        if chars.is_empty() {
            return Err(Error::InvalidSyntax("Provided charset is empty".to_owned()));
        }
        let char_vec: Vec<char> = chars.chars().collect();
        let distr = Uniform::new(0, char_vec.len());
        let distr = distr.map_err(Error::RandomErrorWrapper)?;
        let s: String = (0..length)
            .map(|_| {
                let idx = rng.sample(distr);
                char_vec[idx]
            })
            .collect();
        Ok(s)
    } else {
        let s: String = (0..length)
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect();
        Ok(s)
    }
}

/// Generates a vector of `length` random bytes.
pub fn random_bytes(length: usize) -> Result<Vec<u8>> {
    let mut rng = rng();
    let bytes: Vec<u8> = (0..length).map(|_| rng.random()).collect();
    Ok(bytes)
}

/// Returns an infinite iterator of random values within the inclusive range [min, max].
pub fn random_iter<T>(min: T, max: T) -> Result<impl Iterator<Item = T>>
where
    T: PartialOrd + Copy + SampleUniform + 'static,
{
    let distr = Arc::new(Uniform::new_inclusive(min, max)?);
    let mut rng = rng();
    Ok(repeat_with(move || rng.sample(&*distr)))
}

/// Generates a random value from an inclusive `range`.
pub fn random_range<T>(range: RangeInclusive<T>) -> Result<T>
where
    T: Copy + SampleUniform,
{
    let start = *range.start();
    let end = *range.end();
    let distr = Uniform::new_inclusive(start, end)?;
    let mut rng = rng();
    Ok(rng.sample(distr))
}

/// Returns a random boolean based on `numerator`/`denominator` probability.
pub fn random_ratio(numerator: u32, denominator: u32) -> Result<bool> {
    let mut rng = rng();
    Ok(rng.random_ratio(numerator, denominator))
}
