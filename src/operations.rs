use std::ops::{Add, Sub, Mul, Div};
use crate::length::Length;
use crate::area::Area;
use crate::volume::Volume;


// Implement Add trait
impl Add<Length> for Length {
    type Output = Length;

    fn add(self, other: Length) -> Length {
        Length::Meters(self.to_meters() + other.to_meters())
    }
}

impl Sub<Length> for Length {
    type Output = Length;

    fn sub(self, other: Length) -> Length {
        Length::Meters(self.to_meters() - other.to_meters())
    }
}

impl Add<Area> for Area {
    type Output = Area;

    fn add(self, other: Area) -> Area {
        Area::SquareMeters(self.to_square_meters() + other.to_square_meters())
    }
}

impl Sub<Area> for Area {
    type Output = Area;

    fn sub(self, other: Area) -> Area {
        Area::SquareMeters(self.to_square_meters() - other.to_square_meters())
    }
}

impl Add<Volume> for Volume {
    type Output = Volume;

    fn add(self, other: Volume) -> Volume {
        Volume::Liters(self.to_liters() + other.to_liters())
    }
}

impl Sub<Volume> for Volume {
    type Output = Volume;

    fn sub(self, other: Volume) -> Volume {
        Volume::Liters(self.to_liters() - other.to_liters())
    }
}

// Implement multiplication and division for Length
impl Mul<Length> for Length {
    type Output = Area;
    fn mul(self, other: Length) -> Area {
        Area::SquareMeters(self.to_meters() * other.to_meters())
    }
}

impl Mul<&Length> for &Length {
    type Output = Area;
    fn mul(self, other: &Length) -> Area {
        Area::SquareMeters(self.to_meters() * other.to_meters())
    }
}

impl Div<Length> for Length {
    type Output = f64;
    fn div(self, other: Length) -> f64 {
        self.to_meters() / other.to_meters()
    }
}

// Implement multiplication and division for Area
impl Mul<Length> for Area {
    type Output = Volume;
    fn mul(self, other: Length) -> Volume {
        Volume::Liters(self.to_square_meters() * other.to_meters())
    }
}

impl Div<Length> for Area {
    type Output = Length;
    fn div(self, other: Length) -> Length {
        Length::Meters(self.to_square_meters() / other.to_meters())
    }
}

impl Div<Area> for Area {
    type Output = f64;
    fn div(self, other: Area) -> f64 {
        self.to_square_meters() / other.to_square_meters()
    }
}

// Implement multiplication and division for Volume
impl Mul<Length> for Volume {
    type Output = Volume;
    fn mul(self, other: Length) -> Volume {
        Volume::Liters(self.to_liters() * other.to_meters())
    }
}

impl Div<Area> for Volume {
    type Output = Length;
    fn div(self, other: Area) -> Length {
        Length::Meters(self.to_liters() / other.to_square_meters())
    }
}

impl Div<Length> for Volume {
    type Output = Area;
    fn div(self, other: Length) -> Area {
        Area::SquareMeters(self.to_liters() / other.to_meters())
    }
}

impl Div<Volume> for Volume {
    type Output = f64;
    fn div(self, other: Volume) -> f64 {
        self.to_liters() / other.to_liters()
    }
}

// Implement multiplication and division by scalar for Length
impl Mul<f64> for Length {
    type Output = Length;
    fn mul(self, scalar: f64) -> Length {
        Length::Meters(self.to_meters() * scalar)
    }
}

impl Div<f64> for Length {
    type Output = Length;
    fn div(self, scalar: f64) -> Length {
        Length::Meters(self.to_meters() / scalar)
    }
}

// Implement multiplication and division by scalar for Area
impl Mul<f64> for Area {
    type Output = Area;
    fn mul(self, scalar: f64) -> Area {
        Area::SquareMeters(self.to_square_meters() * scalar)
    }
}

impl Div<f64> for Area {
    type Output = Area;
    fn div(self, scalar: f64) -> Area {
        Area::SquareMeters(self.to_square_meters() / scalar)
    }
}

// Implement multiplication and division by scalar for Volume
impl Mul<f64> for Volume {
    type Output = Volume;
    fn mul(self, scalar: f64) -> Volume {
        Volume::Liters(self.to_liters() * scalar)
    }
}

impl Div<f64> for Volume {
    type Output = Volume;
    fn div(self, scalar: f64) -> Volume {
        Volume::Liters(self.to_liters() / scalar)
    }
}