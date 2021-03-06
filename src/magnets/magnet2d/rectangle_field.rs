/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

//! 2D Magnetic Field Routines
//!
//! This submodule exposes

use crate::magnets::magnet2d::Rectangle;
use crate::utils::points2::{Point2, Points2};
use crate::{FP_CUTOFF, I_2PI, I_4PI};
use std::error::Error;

/// Returns the magnetic field vector due to a rectangle of width `2a`, height  `2b`
/// centered at the origin,
///
/// with an arbitrary magnetisation $`\mathbf{J} = J_x \mathbf{\hat{x}} + J_y \mathbf{\hat{y}}`$
///
pub fn get_field_rectangle(magnet: &Rectangle, point: &Point2) -> Result<Point2, Box<dyn Error>> {
    let mut field = Point2::zero();

    field += if (magnet.jx / magnet.jr).abs() > FP_CUTOFF {
        let local_field = magnetic_field_x(magnet, point);
        match local_field {
            Ok(value) => value,

            // The error will be due to a singularity, so bind  local_field to 0.0
            Err(_e) => Point2 { x: 0.0, y: 0.0 },
        }
    } else {
        Point2 { x: 0.0, y: 0.0 }
    };

    field += if (magnet.jy / magnet.jr).abs() > FP_CUTOFF {
        let local_field = magnetic_field_y(magnet, point);
        match local_field {
            Ok(value) => value,
            Err(_e) => Point2 { x: 0.0, y: 0.0 },
        }
    } else {
        Point2 { x: 0.0, y: 0.0 }
    };

    Ok(field)
}

/// Returns the magnetic field vector at a point due to a rectangle magnetised in x
fn magnetic_field_x(magnet: &Rectangle, point: &Point2) -> Result<Point2, Box<dyn Error>> {
    let field = Point2 {
        x: field_in_x_for_x_mag(point.x, point.y, magnet.a, magnet.b, magnet.jx)?,
        y: field_in_y_for_x_mag(point.x, point.y, magnet.a, magnet.b, magnet.jx)?,
    };
    Ok(field)
}

/// Returns the magnetic field vector at a point due to a rectangle magnetised in y
fn magnetic_field_y(magnet: &Rectangle, point: &Point2) -> Result<Point2, Box<dyn Error>> {
    let field = Point2 {
        x: field_in_x_for_y_mag(point.x, point.y, magnet.a, magnet.b, magnet.jy)?,
        y: field_in_y_for_y_mag(point.x, point.y, magnet.a, magnet.b, magnet.jy)?,
    };
    Ok(field)
}

fn field_in_x_for_x_mag(x: f64, y: f64, a: f64, b: f64, j: f64) -> Result<f64, Box<dyn Error>> {
    // f64.atan2(0.0) = PI/2
    // 1.0_f64.atan2(1.0) = PI/4
    // and thus when J = 1, Bxx = 0.5 when denominator of atan2 is 0.0

    let b_plus_y = b + y;
    let b_minus_y = b - y;
    let b_plus_y_sq = b_plus_y.powi(2);
    let b_minus_y_sq = b_minus_y.powi(2);

    let x_sq = x.powi(2);
    let a_sq = a.powi(2);

    let a2 = 2.0 * a;

    let xsq_minus_a_sq = x_sq - a_sq;

    let top_1 = a2 * b_plus_y;
    let bottom_1 = xsq_minus_a_sq + b_plus_y_sq;

    let top_2 = a2 * (b - y);
    let bottom_2 = xsq_minus_a_sq + b_minus_y_sq;

    Ok(j * I_2PI * (top_1.atan2(bottom_1) + top_2.atan2(bottom_2)))
}

fn field_in_y_for_x_mag(x: f64, y: f64, a: f64, b: f64, j: f64) -> Result<f64, Box<dyn Error>> {
    // when internals of ln = 1, then result  = 0
    // and thus Byx = 0
    let x_plus_a_sq = (x + a).powi(2);
    let x_minus_a_sq = (x - a).powi(2);

    let y_plus_b_sq = (y + b).powi(2);
    let y_minus_b_sq = (y - b).powi(2);

    let top_1 = x_minus_a_sq + y_minus_b_sq;
    let bottom_1 = x_plus_a_sq + y_minus_b_sq;

    let top_2 = x_minus_a_sq + y_plus_b_sq;
    let bottom_2 = x_plus_a_sq + y_plus_b_sq;

    Ok(-j * I_4PI * ((top_1 / bottom_1).ln() - (top_2 / bottom_2).ln()))
}

fn field_in_x_for_y_mag(x: f64, y: f64, a: f64, b: f64, j: f64) -> Result<f64, Box<dyn Error>> {
    // when internals of ln = 1, then result  = 0
    // and thus Bxy = 0
    let x_plus_a_sq = (x + a).powi(2);
    let x_minus_a_sq = (x - a).powi(2);
    let y_plus_b_sq = (y + b).powi(2);
    let y_mins_b_sq = (y - b).powi(2);

    let top_1 = x_plus_a_sq + y_mins_b_sq;
    let bottom_1 = x_plus_a_sq + y_plus_b_sq;

    let top_2 = x_minus_a_sq + y_mins_b_sq;
    let bottom_2 = x_minus_a_sq + y_plus_b_sq;

    Ok(j * I_4PI * ((top_1 / bottom_1).ln() - (top_2 / bottom_2).ln()))
}

fn field_in_y_for_y_mag(x: f64, y: f64, a: f64, b: f64, j: f64) -> Result<f64, Box<dyn Error>> {
    // f64.atan2(0.0) = PI/2
    // 1.0_f64.atan2(1.0) = PI/4
    // and thus when J = 1, Byy = 0.5 when denominator of atan2 is 0.0

    let x_plus_a = x + a;
    let x_minus_a = x - a;

    let x_plus_a_sq = x_plus_a.powi(2);
    let x_minus_a_sq = x_minus_a.powi(2);
    let y_sq = y.powi(2);
    let b_sq = b.powi(2);
    let b2 = 2.0 * b;

    let top_1 = b2 * x_plus_a;
    let bottom_1 = x_plus_a_sq + y_sq - b_sq;

    let top_2 = b2 * x_minus_a;
    let bottom_2 = x_minus_a_sq + y_sq - b_sq;

    Ok(j * I_2PI * (top_1.atan2(bottom_1) - top_2.atan2(bottom_2)))
}

#[cfg(test)]
mod tests {
    use crate::magnets::magnet2d::rectangle_field::get_field_rectangle;
    use crate::magnets::magnet2d::Rectangle;
    use crate::utils::comparison::nearly_equal;
    use crate::utils::points2::Point2;

    #[test]
    fn symmetry_field_in_y() {
        let magnet = Rectangle::new(1.0, 1.0, Point2::new(0., -0.5), 0, 1.0, 90.0);
        let point = Point2::new(0.0, 0.0);
        let field = get_field_rectangle(&magnet, &point).unwrap();
        let result = nearly_equal(field.x, 0.0) && nearly_equal(field.y, 0.5);
        assert!(result);
    }

    #[test]
    fn symmetry_field_in_x() {
        let magnet = Rectangle::new(1.0, 1.0, Point2::new(0., -0.5), 0, 1.0, 0.0);
        let point = Point2::new(0.0, 0.0);
        let field = get_field_rectangle(&magnet, &point).unwrap();
        let result = nearly_equal(field.x, 0.5) && nearly_equal(field.y, 0.0);
        assert!(result);
    }

    #[test]
    fn symmetry_field_45_degree() {
        let magnet = Rectangle::new(1.0, 1.0, Point2::new(0., -0.5), 0, 1.0, 45.0);
        let point = Point2::new(0.0, 0.0);
        let field = get_field_rectangle(&magnet, &point).unwrap();
        let result = nearly_equal(field.x, 0.5 / 2.0_f64.sqrt())
            && nearly_equal(field.y, 0.5 / 2.0_f64.sqrt());
        assert!(result);
    }
}
