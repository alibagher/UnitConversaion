// Define a trait for a generic unit of measure
use std::ops::{Add, Sub, Mul, Div};

pub trait Unit {
    // Returns the symbolic representation of the unit
    fn symbol(&self) -> &str;
}

//PHYSICAL DIMENSIONS
//Dimention::Length
#[derive(Debug, Clone, Copy)]
pub enum Length {
    Megameters(f64),
    Kilometers(f64),
    Hectometers(f64),
    Decameters(f64),
    Meters(f64),
    Decimeters(f64),
    Centimeters(f64),
    Millimeters(f64),
    Micrometers(f64),
    Nanometers(f64),
    Picometers(f64),
    Angstroms(f64),
    Inches(f64),
    Feet(f64),
    Yards(f64),
    Miles(f64),
    ScandinavianMiles(f64),
    LightYears(f64),
    NauticalMiles(f64),
    Fathoms(f64),
    Furlongs(f64),
    AstronomicalUnits(f64),
    Parsecs(f64),
}

impl Unit for Length {
    fn symbol(&self) -> &str {
        match *self {
            Length::Megameters(_) => "Mm",
            Length::Kilometers(_) => "km",
            Length::Hectometers(_) => "hm",
            Length::Decameters(_) => "dam",
            Length::Meters(_) => "m",
            Length::Decimeters(_) => "dm",
            Length::Centimeters(_) => "cm",
            Length::Millimeters(_) => "mm",
            Length::Micrometers(_) => "µm",
            Length::Nanometers(_) => "nm",
            Length::Picometers(_) => "pm",
            Length::Angstroms(_) => "Å",
            Length::Inches(_) => "in",
            Length::Feet(_) => "ft",
            Length::Yards(_) => "yd",
            Length::Miles(_) => "mi",
            Length::ScandinavianMiles(_) => "smi",
            Length::LightYears(_) => "ly",
            Length::NauticalMiles(_) => "NM",
            Length::Fathoms(_) => "ftm",
            Length::Furlongs(_) => "fur",
            Length::AstronomicalUnits(_) => "ua",
            Length::Parsecs(_) => "pc",
        }
    }
}

impl Length {
    pub fn to_meters(&self) -> f64 {
        match *self {
            Length::Megameters(value) => value * 1_000_000.0,
            Length::Kilometers(value) => value * 1000.0,
            Length::Hectometers(value) => value * 100.0,
            Length::Decameters(value) => value * 10.0,
            Length::Meters(value) => value,
            Length::Decimeters(value) => value * 0.1,
            Length::Centimeters(value) => value * 0.01,
            Length::Millimeters(value) => value * 0.001,
            Length::Micrometers(value) => value * 0.000001,
            Length::Nanometers(value) => value * 1e-9,
            Length::Picometers(value) => value * 1e-12,
            Length::Angstroms(value) => value * 1e-10,
            Length::Inches(value) => value * 0.0254,
            Length::Feet(value) => value * 0.3048,
            Length::Yards(value) => value * 0.9144,
            Length::Miles(value) => value * 1609.34,
            Length::ScandinavianMiles(value) => value * 10000.0,
            Length::LightYears(value) => value * 9.461e+15,
            Length::NauticalMiles(value) => value * 1852.0,
            Length::Fathoms(value) => value * 1.8288,
            Length::Furlongs(value) => value * 201.168,
            Length::AstronomicalUnits(value) => value * 1.496e+11,
            Length::Parsecs(value) => value * 3.086e+16,
        }
    }
    
    pub fn to_megameters(&self) -> f64 {
        self.to_meters() / 1_000_000.0
    }

    pub fn to_kilometers(&self) -> f64 {
        self.to_meters() / 1000.0
    }

    pub fn to_hectometers(&self) -> f64 {
        self.to_meters() / 100.0
    }

    pub fn to_decameters(&self) -> f64 {
        self.to_meters() / 10.0
    }

    pub fn to_decimeters(&self) -> f64 {
        self.to_meters() * 10.0
    }

    pub fn to_centimeters(&self) -> f64 {
        self.to_meters() * 100.0
    }

    pub fn to_millimeters(&self) -> f64 {
        self.to_meters() * 1000.0
    }

    pub fn to_micrometers(&self) -> f64 {
        self.to_meters() * 1_000_000.0
    }

    pub fn to_nanometers(&self) -> f64 {
        self.to_meters() * 1e+9
    }

    pub fn to_picometers(&self) -> f64 {
        self.to_meters() * 1e+12
    }

    pub fn to_angstroms(&self) -> f64 {
        self.to_meters() * 1e+10
    }

    pub fn to_inches(&self) -> f64 {
        self.to_meters() / 0.0254
    }

    pub fn to_feet(&self) -> f64 {
        self.to_meters() / 0.3048
    }

    pub fn to_yards(&self) -> f64 {
        self.to_meters() / 0.9144
    }

    pub fn to_miles(&self) -> f64 {
        self.to_meters() / 1609.34
    }

    pub fn to_scandinavian_miles(&self) -> f64 {
        self.to_meters() / 10000.0
    }

    pub fn to_light_years(&self) -> f64 {
        self.to_meters() / 9.461e+15
    }

    pub fn to_nautical_miles(&self) -> f64 {
        self.to_meters() / 1852.0
    }

    pub fn to_fathoms(&self) -> f64 {
        self.to_meters() / 1.8288
    }

    pub fn to_furlongs(&self) -> f64 {
        self.to_meters() / 201.168
    }

    pub fn to_astronomical_units(&self) -> f64 {
        self.to_meters() / 1.496e+11
    }

    pub fn to_parsecs(&self) -> f64 {
        self.to_meters() / 3.086e+16
    }

}

//Dimention: Area
#[derive(Debug, Clone, Copy)]
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

impl Unit for Area {
    fn symbol(&self) -> &str {
        match *self {
            Area::SquareMegameters(_) => "Mm²",
            Area::SquareKilometers(_) => "km²",
            Area::SquareMeters(_) => "m²",
            Area::SquareCentimeters(_) => "cm²",
            Area::SquareMillimeters(_) => "mm²",
            Area::SquareMicrometers(_) => "µm²",
            Area::SquareNanometers(_) => "nm²",
            Area::SquareInches(_) => "in²",
            Area::SquareFeet(_) => "ft²",
            Area::SquareYards(_) => "yd²",
            Area::SquareMiles(_) => "mi²",
            Area::Acres(_) => "ac",
            Area::Ares(_) => "a",
            Area::Hectares(_) => "ha",
        }
    }
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


//OPERATIONS:
// Implement Add trait
impl Add for Length {
    type Output = Length;

    fn add(self, other: Length) -> Length {
        Length::Meters(self.to_meters() + other.to_meters())
    }
}

// Implement Sub trait
impl Sub for Length {
    type Output = Length;

    fn sub(self, other: Length) -> Length {
        Length::Meters(self.to_meters() - other.to_meters())
    }
}

impl Mul for Length {
    type Output = Area;

    fn mul(self, other: Length) -> Area {
        Area::SquareMeters(self.to_meters() * other.to_meters())
    }
}

impl Div for Length {
    type Output = f64;

    fn div(self, other: Length) -> f64 {
        self.to_meters() / other.to_meters()
    }
}