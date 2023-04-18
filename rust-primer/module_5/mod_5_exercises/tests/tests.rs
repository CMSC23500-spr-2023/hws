use mod_5_exercises::solution::{Geometry, Rectangle, Circle};
use std::f64::consts::PI;

#[test]
fn check_rect_area1() {
    let x = Rectangle {length: 3.0, width: 1.0};
    assert!(nearly_equal(x.get_area(), 3.0));
}

#[test]
fn check_rect_area2() {
    let x = Rectangle {length: 3.5, width: 2.2};
    assert!(nearly_equal(x.get_area(), 7.7));
}

#[test]
fn check_circle_area1() {
    let x = Circle {radius: 1.0 };
    let expected = PI;
    assert!(nearly_equal(x.get_area(), expected));
}

#[test]
fn check_circle_area2() {
    let x = Circle {radius: 2.5 };
    let expected = 2.5 * 2.5 * PI;
    assert!(nearly_equal(x.get_area(), expected));
}

// An ok impl of a check float with delta.
// See https://floating-point-gui.de/ &&  https://www.itu.dk/~sestoft/bachelor/IEEE754_article.pdf
// Could have used a crate https://docs.rs/approx/0.3.2/approx/
// this is taken from https://users.rust-lang.org/t/assert-eq-for-float-numbers/7034/3
pub fn nearly_equal(a: f64, b: f64) -> bool {
	let abs_a = a.abs();
	let abs_b = b.abs();
	let diff = (a - b).abs();

	if a == b { // Handle infinities.
		true
	} else if a == 0.0 || b == 0.0 || diff < f64::MIN_POSITIVE {
		// One of a or b is zero (or both are extremely close to it,) use absolute error.
		diff < (f64::EPSILON * f64::MIN_POSITIVE)
	} else { // Use relative error.
		(diff / f64::min(abs_a + abs_b, f64::MAX)) < f64::EPSILON
	}
}
