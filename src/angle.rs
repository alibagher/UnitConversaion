use std::fmt;
use std::ops::{Add, Sub,Div};

#[derive(Clone, Copy)]
pub enum Angle {
    Revolutions(f64),
    Degrees(f64),
    Radians(f64),
    Gradians(f64),
}

impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Angle::Revolutions(value) => write!(f, "{} rev", value),
            Angle::Degrees(value) => write!(f, "{}°", value),
            Angle::Radians(value) => write!(f, "{} rad", value),
            Angle::Gradians(value) => write!(f, "{} grad", value),
        }
    }
}

impl Angle {
    pub fn to_degrees(&self) -> f64 {
        match *self {
            Angle::Revolutions(value) => value * 360.0,
            Angle::Degrees(value) => value,
            Angle::Radians(value) => value * 180.0 / std::f64::consts::PI,
            Angle::Gradians(value) => value * 0.9,
        }
    }

    pub fn to_radians(&self) -> f64 {
        match *self {
            Angle::Revolutions(value) => value * 2.0 * std::f64::consts::PI,
            Angle::Degrees(value) => value * std::f64::consts::PI / 180.0,
            Angle::Radians(value) => value,
            Angle::Gradians(value) => value * std::f64::consts::PI / 200.0,
        }
    }

    pub fn to_revolutions(&self) -> f64 {
        match *self {
            Angle::Revolutions(value) => value,
            Angle::Degrees(value) => value / 360.0,
            Angle::Radians(value) => value / (2.0 * std::f64::consts::PI),
            Angle::Gradians(value) => value / 400.0,
        }
    }

    pub fn to_gradians(&self) -> f64 {
        match *self {
            Angle::Revolutions(value) => value * 400.0,
            Angle::Degrees(value) => value / 0.9,
            Angle::Radians(value) => value * 200.0 / std::f64::consts::PI,
            Angle::Gradians(value) => value,
        }
    }
}


//TODO: Angle Operations
impl Add<Angle> for Angle {
    type Output = Angle;
    fn add(self, other: Angle) -> Angle {
        Angle::Degrees(self.to_degrees() + other.to_degrees())
    }
}

impl Sub<Angle> for Angle {
    type Output = Angle;
    fn sub(self, other: Angle) -> Angle {
        Angle::Degrees(self.to_degrees() - other.to_degrees())
    }
}

impl Div<Angle> for Angle {
    type Output = f64;
    fn div(self, other: Angle) -> f64 {
        self.to_degrees() / other.to_degrees()
    }
}

//TODO: Implement Degres Squared ? 