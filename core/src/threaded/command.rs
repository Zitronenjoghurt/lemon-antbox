pub enum SimulationCommand {
    Clear,
    Shutdown,
    ForceStep,
    Inspect {
        x: u16,
        y: u16,
    },
    SpawnAnt {
        x: u16,
        y: u16,
        tribe: u8,
        radius: u8,
    },
    SpawnNest {
        x: u16,
        y: u16,
        tribe: u8,
        radius: u8,
    },
    SpawnFood {
        x: u16,
        y: u16,
        amount: u8,
        radius: u8,
    },
}
