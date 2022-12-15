// Some values in these datasets are flagged as similar to defined f32 constants.
// This directive makes clippy ignore these false positives.
#![allow(clippy::approx_constant)]

pub mod breast_cancer;
pub mod diabetes;
pub mod digits;
pub mod iris;
pub mod wine;
