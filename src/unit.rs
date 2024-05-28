use std::fmt;

use std::ops::{Add, Sub, Mul, Div};

//////////////PHYSICAL DIMENSIONS/////////////////////

//Dimension::Length
#[derive(Clone, Copy)]
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

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Length::Megameters(value) => write!(f, "{} Mm", value),
            Length::Kilometers(value) => write!(f, "{} km", value),
            Length::Hectometers(value) => write!(f, "{} hm", value),
            Length::Decameters(value) => write!(f, "{} dam", value),
            Length::Meters(value) => write!(f, "{} m", value),
            Length::Decimeters(value) => write!(f, "{} dm", value),
            Length::Centimeters(value) => write!(f, "{} cm", value),
            Length::Millimeters(value) => write!(f, "{} mm", value),
            Length::Micrometers(value) => write!(f, "{} µm", value),
            Length::Nanometers(value) => write!(f, "{} nm", value),
            Length::Picometers(value) => write!(f, "{} pm", value),
            Length::Angstroms(value) => write!(f, "{} Å", value),
            Length::Inches(value) => write!(f, "{} in", value),
            Length::Feet(value) => write!(f, "{} ft", value),
            Length::Yards(value) => write!(f, "{} yd", value),
            Length::Miles(value) => write!(f, "{} mi", value),
            Length::ScandinavianMiles(value) => write!(f, "{} smi", value),
            Length::LightYears(value) => write!(f, "{} ly", value),
            Length::NauticalMiles(value) => write!(f, "{} NM", value),
            Length::Fathoms(value) => write!(f, "{} ftm", value),
            Length::Furlongs(value) => write!(f, "{} fur", value),
            Length::AstronomicalUnits(value) => write!(f, "{} AU", value),
            Length::Parsecs(value) => write!(f, "{} pc", value),
        }
    }
}

//Dimension::Area
#[derive( Clone, Copy)]
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

//Dimension::Volume
#[derive(Clone, Copy)]
pub enum Volume {
    Megaliters(f64),
    Kiloliters(f64),
    Liters(f64),
    Deciliters(f64),
    Centiliters(f64),
    Milliliters(f64),
    CubicKilometers(f64),
    CubicMeters(f64),
    CubicDecimeters(f64),
    CubicMillimeters(f64),
    CubicInches(f64),
    CubicFeet(f64),
    CubicYards(f64),
    CubicMiles(f64),
    AcreFeet(f64),
    Bushels(f64),
    Teaspoons(f64),
    Tablespoons(f64),
    FluidOunces(f64),
    Cups(f64),
    Pints(f64),
    Quarts(f64),
    Gallons(f64),
    ImperialTeaspoons(f64),
    ImperialTablespoons(f64),
    ImperialFluidOunces(f64),
    ImperialPints(f64),
    ImperialQuarts(f64),
    ImperialGallons(f64),
    MetricCups(f64),
}

impl Volume {
    pub fn to_liters(&self) -> f64 {
        match *self {
            Volume::Megaliters(value) => value * 1_000_000.0,
            Volume::Kiloliters(value) => value * 1000.0,
            Volume::Liters(value) => value,
            Volume::Deciliters(value) => value * 0.1,
            Volume::Centiliters(value) => value * 0.01,
            Volume::Milliliters(value) => value * 0.001,
            Volume::CubicKilometers(value) => value * 1e12,
            Volume::CubicMeters(value) => value * 1000.0,
            Volume::CubicDecimeters(value) => value,
            Volume::CubicMillimeters(value) => value * 1e-6,
            Volume::CubicInches(value) => value * 0.0163871,
            Volume::CubicFeet(value) => value * 28.3168,
            Volume::CubicYards(value) => value * 764.555,
            Volume::CubicMiles(value) => value * 4.168e+12,
            Volume::AcreFeet(value) => value * 1.233e+6,
            Volume::Bushels(value) => value * 35.2391,
            Volume::Teaspoons(value) => value * 0.00492892,
            Volume::Tablespoons(value) => value * 0.0147868,
            Volume::FluidOunces(value) => value * 0.0295735,
            Volume::Cups(value) => value * 0.24,
            Volume::Pints(value) => value * 0.473176,
            Volume::Quarts(value) => value * 0.946353,
            Volume::Gallons(value) => value * 3.78541,
            Volume::ImperialTeaspoons(value) => value * 0.00591939,
            Volume::ImperialTablespoons(value) => value * 0.0177582,
            Volume::ImperialFluidOunces(value) => value * 0.0284131,
            Volume::ImperialPints(value) => value * 0.568261,
            Volume::ImperialQuarts(value) => value * 1.13652,
            Volume::ImperialGallons(value) => value * 4.54609,
            Volume::MetricCups(value) => value * 0.25,
        }
    }
}

impl fmt::Display for Volume {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Volume::Megaliters(value) => write!(f, "{} Ml", value),
            Volume::Kiloliters(value) => write!(f, "{} kL", value),
            Volume::Liters(value) => write!(f, "{} L", value),
            Volume::Deciliters(value) => write!(f, "{} dL", value),
            Volume::Centiliters(value) => write!(f, "{} cL", value),
            Volume::Milliliters(value) => write!(f, "{} ml", value),
            Volume::CubicKilometers(value) => write!(f, "{} km³", value),
            Volume::CubicMeters(value) => write!(f, "{} m³", value),
            Volume::CubicDecimeters(value) => write!(f, "{} dm³", value),
            Volume::CubicMillimeters(value) => write!(f, "{} mm³", value),
            Volume::CubicInches(value) => write!(f, "{} in³", value),
            Volume::CubicFeet(value) => write!(f, "{} ft³", value),
            Volume::CubicYards(value) => write!(f, "{} yd³", value),
            Volume::CubicMiles(value) => write!(f, "{} mi³", value),
            Volume::AcreFeet(value) => write!(f, "{} acre-ft", value),
            Volume::Bushels(value) => write!(f, "{} bu", value),
            Volume::Teaspoons(value) => write!(f, "{} tsp", value),
            Volume::Tablespoons(value) => write!(f, "{} tbsp", value),
            Volume::FluidOunces(value) => write!(f, "{} fl oz", value),
            Volume::Cups(value) => write!(f, "{} cup", value),
            Volume::Pints(value) => write!(f, "{} pt", value),
            Volume::Quarts(value) => write!(f, "{} qt", value),
            Volume::Gallons(value) => write!(f, "{} gal", value),
            Volume::ImperialTeaspoons(value) => write!(f, "{} tsp", value),
            Volume::ImperialTablespoons(value) => write!(f, "{} tbsp", value),
            Volume::ImperialFluidOunces(value) => write!(f, "{} fl oz", value),
            Volume::ImperialPints(value) => write!(f, "{} pt", value),
            Volume::ImperialQuarts(value) => write!(f, "{} qt", value),
            Volume::ImperialGallons(value) => write!(f, "{} gal", value),
            Volume::MetricCups(value) => write!(f, "{} metric cup", value),
        }
    }
}


//OPERATIONS:
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