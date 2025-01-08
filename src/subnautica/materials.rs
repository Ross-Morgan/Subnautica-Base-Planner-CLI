use std::collections::HashMap;

use strum::{AsRefStr, EnumIter, EnumString};

#[derive(
    Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, AsRefStr, EnumIter, EnumString,
)]
#[strum(serialize_all = "lowercase")]
pub enum Item {
    Bulkhead,
    Foundation,
    GlassCompartment,
    TitaniumCompartment,
    Hatch,
    Moonpool,
    Observatory,
    Reinforcement,
    ScannerRoom,
    VerticalConnector,
    WaterFiltrationMachine,
    Window,
    MultipurposeRoom,
    MultipurposeRoomGlassRoof,
    LargeRoom,
    LargeRoomGlassRoof,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, EnumIter, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Material {
    Aerogel,
    Copper,
    CopperWire,
    EnamledGlass,
    Gold,
    Glass,
    Lead,
    Lithium,
    Lubricant,
    PlasteelIngot,
    Quartz,
    SiliconeRubber,
    TableCoral,
    Titanium,
    TitaniumIngot,
}

impl Item {
    #[must_use]
    pub fn materials(self) -> HashMap<Material, usize> {
        match self {
            Item::Bulkhead => {
                HashMap::from([(Material::Titanium, 3), (Material::SiliconeRubber, 1)])
            }
            Item::Foundation => HashMap::from([(Material::Lead, 2), (Material::Titanium, 2)]),
            Item::GlassCompartment | Item::Window => HashMap::from([(Material::Glass, 2)]),
            Item::TitaniumCompartment | Item::VerticalConnector => {
                HashMap::from([(Material::Titanium, 2)])
            }
            Item::Hatch => HashMap::from([(Material::Titanium, 2), (Material::Quartz, 1)]),
            Item::Moonpool => HashMap::from([
                (Material::TitaniumIngot, 2),
                (Material::Lubricant, 1),
                (Material::Lead, 2),
            ]),
            Item::Observatory => {
                HashMap::from([(Material::EnamledGlass, 2), (Material::Titanium, 1)])
            }
            Item::Reinforcement => HashMap::from([(Material::Titanium, 3), (Material::Lithium, 1)]),
            Item::ScannerRoom => HashMap::from([
                (Material::Titanium, 5),
                (Material::Copper, 2),
                (Material::Gold, 1),
                (Material::TableCoral, 1),
            ]),
            Item::WaterFiltrationMachine => HashMap::from([
                (Material::Titanium, 3),
                (Material::CopperWire, 1),
                (Material::Aerogel, 1),
            ]),
            Item::MultipurposeRoom => HashMap::from([(Material::Titanium, 6)]),
            Item::MultipurposeRoomGlassRoof => HashMap::from([
                (Material::EnamledGlass, 2),
                (Material::Titanium, 2),
                (Material::Lithium, 1),
            ]),
            Item::LargeRoom => HashMap::from([(Material::PlasteelIngot, 2)]),
            Item::LargeRoomGlassRoof => HashMap::from([
                (Material::EnamledGlass, 4),
                (Material::Titanium, 4),
                (Material::Lithium, 2),
            ]),
        }
    }
}
