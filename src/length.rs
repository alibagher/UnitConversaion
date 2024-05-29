use::std::fmt;

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
