use super::materials::Item;

pub trait Integrity {
    fn integrity(&self) -> f64;
}

impl Integrity for Item {
    fn integrity(&self) -> f64 {
        match *self {
            Item::Bulkhead => 3.0,
            Item::Foundation => 2.0,
            Item::GlassCompartment => -2.0,
            Item::TitaniumCompartment => -1.0,
            Item::Hatch => -1.0,
            Item::Moonpool => -5.0,
            Item::Observatory => -3.0,
            Item::Reinforcement => 7.0,
            Item::ScannerRoom => -1.0,
            Item::VerticalConnector => -0.5,
            Item::WaterFiltrationMachine => -2.0,
            Item::Window => -1.0,
            Item::MultipurposeRoom => -1.25,
            Item::MultipurposeRoomGlassRoof => -2.0,
            Item::LargeRoom => -4.0,
            Item::LargeRoomGlassRoof => -4.0,
        }
    }
}