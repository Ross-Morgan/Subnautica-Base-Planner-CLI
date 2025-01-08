use super::base::Base;
use super::materials::Item;

pub trait Integrity {
    fn integrity(&self) -> f64;
}

impl Integrity for Item {
    fn integrity(&self) -> f64 {
        match *self {
            Item::Bulkhead => 3.0,
            Item::Foundation => 2.0,
            Item::TitaniumCompartment | Item::Hatch | Item::Window | Item::ScannerRoom => -1.0,
            Item::Moonpool => -5.0,
            Item::Observatory => -3.0,
            Item::Reinforcement => 7.0,
            Item::VerticalConnector => -0.5,
            Item::MultipurposeRoom => -1.25,
            Item::GlassCompartment | Item::WaterFiltrationMachine | Item::MultipurposeRoomGlassRoof => -2.0,
            Item::LargeRoom | Item::LargeRoomGlassRoof => -4.0,
        }
    }
}

#[allow(clippy::cast_precision_loss, clippy::cast_sign_loss)]
impl Integrity for Base {
    fn integrity(&self) -> f64 {
        let negative_integrity = self
            .items
            .iter()
            .filter_map(|(item, count)| match item.integrity() {
                i if i < 0.0 => Some(i * (*count as f64)),
                _ => None,
            })
            .sum::<f64>();

        let positive_integrity = self
            .items
            .iter()
            .filter_map(|(item, count)| match item.integrity() {
                i if i > 0.0 => Some(i * (*count as f64)),
                _ => None,
            })
            .sum::<f64>();

        let multiplier = match self.depth {
            0 => 0.0,
            1..=99 => 1.0,
            depth => f64::from((depth - 100) / 1000) + 1.0,
        };

        ((negative_integrity + 10.0) * multiplier) + positive_integrity
    }
}
