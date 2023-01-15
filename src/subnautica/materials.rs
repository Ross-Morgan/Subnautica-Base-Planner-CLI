use strum_macros::EnumString;
use strum;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, EnumString)]
#[strum(serialize_all = "PascalCase")]
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

pub enum Material {
    Titanium,
    Glass,
    EnamledGlass,
    Lithium,
    SiliconeRubber,
    CopperWire,
}
