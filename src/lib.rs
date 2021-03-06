// #![warn(missing_docs)]
//! This test library and binary implements routines for calculating magnetic
//! fields, written in Rust. A more complete Python version can be found on
//! [Github](https://github.com/pdunne/pymagnet), or
//! [PyPi](https://pypi.org/project/pymagnet/)
//!
//! # User friendly magnetic field calculations
//! This library consists of methods for calculating magnetic fields due
//! to simple objects in 2D and 3D.
//!
//!
//!
//! # Calculation Method
//! ## Exact Analytical Methods
//!
//! ## Iterative Method for Cylindrical Sources

use core::f64;

pub mod magnets;
pub mod utils;

/// Non a number - float64 variant
pub const NAN: f64 = f64::NAN;

/// PI
pub const PI: f64 = std::f64::consts::PI;
/// 2*PI
pub const M2_PI: f64 = PI * 2.0;
/// 4*PI
pub const M4_PI: f64 = PI * 4.0;
/// PI/2
pub const PI_2: f64 = PI / 2.0;
/// PI/3
pub const PI_3: f64 = PI / 3.0;
/// PI/4
pub const PI_4: f64 = PI / 4.0;
/// PI/4
pub const PI_6: f64 = PI / 6.0;
/// 1/(2*PI)
pub const I_2PI: f64 = 1.0 / M2_PI;
/// 1/(2*PI)
pub const I_4PI: f64 = 1.0 / M4_PI;
/// Floating point cutoff for vector alignment
pub const FP_CUTOFF: f64 = 1e-6;

/// Floating point cutoff for relative error
pub const ERR_CUTOFF: f64 = 1e-12;
