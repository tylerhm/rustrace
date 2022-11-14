use crate::core::primitive::Primitive;
use crate::core::scene::Scene;

/////////////////////
// BEGIN INTERFACE //
/////////////////////

/// An [Accelerator] is responsible for testing rays against a [Scene].
/// The most basic implementation would test every [Primitive] in the [Scene]
/// and return the closest.
pub trait Accelerator {}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

////////////////////////
// END IMPLEMENTATION //
// BEGIN TESTS        //
////////////////////////

///////////////
// END TESTS //
///////////////
