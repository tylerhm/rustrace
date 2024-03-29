use std::cmp::{Ordering, PartialEq, PartialOrd};

use crate::core::vector::{Point3f, Vec3f};

/////////////////////
// BEGIN INTERFACE //
/////////////////////

/// Represents the collision of light with some participating medium.
#[derive(Debug, Clone, Copy)]
pub struct Interaction {
    /// The location of the hit
    pub p: Point3f,

    /// The time of the hit, represents a scalar on the unit ray
    pub t: f64,

    /// The normal of the surface at the hit location
    /// Optional to account for media collisions
    pub n: Option<Vec3f>,

    /// The wo term as represented in the rendering equation
    pub wo: Vec3f,
}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

impl Interaction {
    pub fn new_on_surface(p: Point3f, t: f64, n: Vec3f, wo: Vec3f) -> Self {
        Self {
            p,
            t,
            n: Some(n),
            wo,
        }
    }

    pub fn new_in_media(p: Point3f, t: f64, wo: Vec3f) -> Self {
        Self { p, t, n: None, wo }
    }

    pub fn is_eq(&self, other: &Self) -> bool {
        self.p == other.p && self.t == other.t && self.n == other.n && self.wo == other.wo
    }
}

impl PartialOrd for Interaction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

impl PartialEq for Interaction {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

////////////////////////
// END IMPLEMENTATION //
// BEGIN TESTS        //
////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ord() {
        let a = Interaction::new_on_surface(
            Point3f::new(1.0, 1.0, 1.0),
            0.5,
            Vec3f::new(1.0, 1.0, 1.0),
            Vec3f::new(1.0, 1.0, 1.0),
        );

        let b = Interaction::new_on_surface(
            Point3f::new(1.0, 1.0, 1.0),
            0.6,
            Vec3f::new(1.0, 1.0, 1.0),
            Vec3f::new(1.0, 1.0, 1.0),
        );

        assert!(a < b);
        assert!(b > a);
        assert!(a == a);
    }
}

///////////////
// END TESTS //
///////////////
