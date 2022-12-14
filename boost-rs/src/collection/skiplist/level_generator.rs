//! SkipLists use a probabilistic distribution of nodes over the internal
//! levels, whereby the lowest level (level 0) contains all the nodes, and each
//! level `n > 0` will contain a random subset of the nodes on level `n - 1`.
//!
//! Most commonly, a geometric distribution is used whereby the chance that a
//! node occupies level `n` is `p` times the chance of occupying level `n-1`
//! (with `0 < p < 1`).
//!
//! It is very unlikely that this will need to be changed as the default should
//! suffice, but if need be custom level generators can be implemented.

use rand::rngs::SmallRng;
use rand::{thread_rng, Rng, SeedableRng};

use crate::collection::error::CollectionError;

/// Upon the insertion of a new node in the list, the node is replicated to high
/// levels with a certain probability as determined by a `LevelGenerator`.
pub trait GenerateLevel {
    /// The max number of levels that are assumed to exist for this level
    /// generator.
    fn level_bound(&self) -> usize;
    /// Generate a random level for a new node in the range `[0, level_bound)`.
    ///
    /// This must never return a level that is `>= self.level_bound)`.
    fn random(&mut self) -> usize;
}

/// A level generator which will produce geometrically distributed numbers.
///
/// The probability of generating level `n` is `p` times the probability of
/// generating level `n-1`, with the probability truncated at the maximum number
/// of levels allowed.
pub struct DefaultLevelGenerator {
    level_bound: usize,
    p: f64,
    // unit_range: distributions::Range<f64>,
    rng: SmallRng,
}

impl Default for DefaultLevelGenerator {
    fn default() -> Self {
        DefaultLevelGenerator::new(16, 1.0 / 2.0).unwrap()
    }
}

impl DefaultLevelGenerator {
    /// Create a new GeometricalLevelGenerator with `level_bound` number of levels,
    /// and `p` as the probability that a given node is present in the next
    /// level.
    ///
    /// `p` must be between 0 and 1 and will panic otherwise. Similarly,
    /// `level_bound` must be at greater or equal to 1.
    pub fn new(level_bound: usize, p: f64) -> Result<Self, CollectionError> {
        if level_bound == 0 {
            return Err(CollectionError::InvalidParameter(
                "total must be non-zero.".to_string(),
            ));
        }
        if (p - 0.0).abs() < 1e-3 || (p - 1.0).abs() < 1e-3 {
            return Err(CollectionError::InvalidParameter(
                "p must be in (0,1).".to_string(),
            ));
        }
        Ok(DefaultLevelGenerator {
            level_bound,
            p,
            // unit_range: distributions::Range::new(0.0f64, 1.0),
            rng: SmallRng::from_rng(thread_rng()).unwrap(),
        })
    }
}

impl GenerateLevel for DefaultLevelGenerator {
    fn level_bound(&self) -> usize {
        self.level_bound
    }

    fn random(&mut self) -> usize {
        let mut level = 1;
        let mut x = self.p;
        let f = 1.0 - self.rng.gen::<f64>();
        while x > f && level + 1 < self.level_bound {
            level += 1;
            x *= self.p
        }
        level
    }
}

#[cfg(test)]
mod tests {
    use crate::collection::skiplist::level_generator::{DefaultLevelGenerator, GenerateLevel};

    #[test]
    fn invalid_total() {
        assert!(DefaultLevelGenerator::new(0, 0.5).is_err())
    }

    #[test]
    fn invalid_p_0() {
        let res = DefaultLevelGenerator::new(1, 0.0).is_err();
        println!("res: {}", res);
        assert!(res)
    }

    #[test]
    fn invalid_p_1() {
        assert!(DefaultLevelGenerator::new(1, 1.0).is_err())
    }

    #[test]
    fn new() {
        DefaultLevelGenerator::new(1, 0.5).unwrap();
    }

    #[test]
    fn random() {
        let level_bound = 5;
        let mut g = DefaultLevelGenerator::new(level_bound, 0.5).unwrap();
        for _ in 0..10 {
            let level = g.random();
            println!("current level: {}", level);
            assert!(level < level_bound);
        }
    }
}
