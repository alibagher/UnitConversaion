use std::fmt;

pub enum Area {
    SquareMegameters(f64),
    SquareKilometers(f64),
    SquareMeters(f64),
    SquareCentimeters(f64),
    SquareMillimeters(f64),
    SquareMicrometers(f64),
    SquareNanometers(f64),
    SquareInches(f64),
    SquareFeet(f64),
    SquareYards(f64),
    SquareMiles(f64),
    Acres(f64),
    Ares(f64),
    Hectares(f64),
}

impl Area {
    pub fn to_square_meters(&self) -> f64 {
        match *self {
            Area::SquareMegameters(value) => value * 1_000_000_000.0,
            Area::SquareKilometers(value) => value * 1_000_000.0,
            Area::SquareMeters(value) => value,
            Area::SquareCentimeters(value) => value * 0.0001,
            Area::SquareMillimeters(value) => value * 0.000001,
            Area::SquareMicrometers(value) => value * 1e-12,
            Area::SquareNanometers(value) => value * 1e-18,
            Area::SquareInches(value) => value * 0.00064516,
            Area::SquareFeet(value) => value * 0.092903,
            Area::SquareYards(value) => value * 0.836127,
            Area::SquareMiles(value) => value * 2.59e+6,
            Area::Acres(value) => value * 4046.86,
            Area::Ares(value) => value * 100.0,
            Area::Hectares(value) => value * 10_000.0,
        }
    }

    pub fn to_square_megameters(&self) -> f64 {
        self.to_square_meters() / 1_000_000_000.0
    }

    pub fn to_square_kilometers(&self) -> f64 {
        self.to_square_meters() / 1_000_000.0
    }

    pub fn to_square_centimeters(&self) -> f64 {
        self.to_square_meters() * 0.0001
    }

    pub fn to_square_millimeters(&self) -> f64 {
        self.to_square_meters() * 0.000001
    }

    pub fn to_square_micrometers(&self) -> f64 {
        self.to_square_meters() * 1e-12
    }

    pub fn to_square_nanometers(&self) -> f64 {
        self.to_square_meters() * 1e-18
    }

    pub fn to_square_inches(&self) -> f64 {
        self.to_square_meters() / 0.00064516
    }

    pub fn to_square_feet(&self) -> f64 {
        self.to_square_meters() / 0.092903
    }

    pub fn to_square_yards(&self) -> f64 {
        self.to_square_meters() / 0.836127
    }
    
    pub fn to_square_miles(&self) -> f64 {
        self.to_square_meters() / 2.59e+6
    }

    pub fn to_acres(&self) -> f64 {
        self.to_square_meters() / 4046.86
    }

    pub fn to_ares(&self) -> f64 {
        self.to_square_meters() / 100.0
    }

    pub fn to_hectares(&self) -> f64 {
        self.to_square_meters() / 10_000.0
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Area::SquareMegameters(value) => write!(f, "{} Mm²", value),
            Area::SquareKilometers(value) => write!(f, "{} km²", value),
            Area::SquareMeters(value) => write!(f, "{} m²", value),
            Area::SquareCentimeters(value) => write!(f, "{} cm²", value),
            Area::SquareMillimeters(value) => write!(f, "{} mm²", value),
            Area::SquareMicrometers(value) => write!(f, "{} µm²", value),
            Area::SquareNanometers(value) => write!(f, "{} nm²", value),
            Area::SquareInches(value) => write!(f, "{} in²", value),
            Area::SquareFeet(value) => write!(f, "{} ft²", value),
            Area::SquareYards(value) => write!(f, "{} yd²", value),
            Area::SquareMiles(value) => write!(f, "{} mi²", value),
            Area::Acres(value) => write!(f, "{} ac", value),
            Area::Ares(value) => write!(f, "{} a", value),
            Area::Hectares(value) => write!(f, "{} ha", value),
        }
    }
}
