#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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
