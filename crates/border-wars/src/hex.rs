//! All functions related to calculations in a hexagonal grid.

use std::collections::HashSet;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use num::{FromPrimitive, Signed};
use partial_min_max::{max, min};

/// Represents a number that can be used in a hexagonal grid.
pub trait HexNumber: Signed + PartialEq + Copy + PartialOrd + FromPrimitive {}

impl<T: Signed + PartialEq + Copy + PartialOrd + FromPrimitive> HexNumber for T {}

/// Represents a position in a hexagonal grid.
/// We use the axial coordinate system explained in this
/// [documentation](https://www.redblobgames.com/grids/hexagons/#coordinates).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HexPosition<T: HexNumber> {
    /// Q coordinate.
    pub q: T,

    /// R coordinate.
    pub r: T,
}

impl<T: HexNumber> HexPosition<T> {
    /// Returns the distance between two [HexPosition]s.
    ///
    /// # How it works
    ///
    /// In the hexagonal grid, using the
    /// [cube coordinate system](https://www.redblobgames.com/grids/hexagons/#coordinates),
    /// it's akin to a cube in 3D space.
    /// The Manhattan distance between two positions is equal to half of
    /// the sum of abs(dx) + abs(dy) + abs(dz).
    /// However, in hexagonal grids, z is defined as -q - r.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use border_wars::hex::HexPosition;
    ///
    /// let a = HexPosition { q: 0, r: 0 };
    /// let b = HexPosition { q: 1, r: 1 };
    ///
    /// assert_eq!(a.distance_to(&b), 2);
    /// ```
    pub fn distance_to(&self, other: &Self) -> T {
        // Calculate the difference between the q and r coordinates.
        let dq = (self.q - other.q).abs();
        let dr = (self.r - other.r).abs();
        let ds = dq + dr;

        // Manhattan distance = (abs(dq) + abs(dr) + abs(ds)) / 2
        (dq + dr + ds) / (T::one() + T::one())
    }
}

impl<T: HexNumber + Eq + Hash + std::cmp::PartialOrd + num::ToPrimitive> HexPosition<T> {
    /// Returns all positions within a given `range` from the current
    /// `HexPosition`.
    ///
    /// This function iterates over the possible q and r values within the
    /// specified range.
    /// Note that the original position is also returned.
    ///
    /// For more details, refer to: https://www.redblobgames.com/grids/hexagons/#range
    ///
    /// # Example
    ///
    /// ```
    /// use border_wars::hex::HexPosition;
    ///
    /// let position = HexPosition { q: 0, r: 0 };
    ///
    /// let positions = position.range(1);
    ///
    /// assert_eq!(positions.len(), 7);
    /// ```
    pub fn range(&self, range: T) -> HashSet<Self> {
        let mut result_positions = HashSet::new();
        for q in num::range_inclusive(-range, range) {
            for r in num::range_inclusive(max(-range, -q - range), min(range, -q + range)) {
                result_positions.insert(Self { q, r });
            }
        }
        result_positions
    }
}

impl<T: HexNumber> Add<Self> for HexPosition<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            q: self.q + other.q,
            r: self.r + other.r,
        }
    }
}

impl<T: HexNumber + AddAssign> AddAssign<Self> for HexPosition<T> {
    fn add_assign(&mut self, other: Self) {
        self.q += other.q;
        self.r += other.r;
    }
}

impl<T: HexNumber> Sub<Self> for HexPosition<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            q: self.q - other.q,
            r: self.r - other.r,
        }
    }
}

impl<T: HexNumber + SubAssign> SubAssign<Self> for HexPosition<T> {
    fn sub_assign(&mut self, other: Self) {
        self.q -= other.q;
        self.r -= other.r;
    }
}
