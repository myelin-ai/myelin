#![feature(proc_macro_hygiene)]

use std::f64::consts::PI;
use radians_macro::radians;

#[test]
fn works_with_0() {
    radians!(0.0);
}

#[test]
fn works_with_PI() {
    radians!(PI);
}

#[test]
fn works_with_1_point_9_PI() {
    radians!(1.9 * PI);
}
