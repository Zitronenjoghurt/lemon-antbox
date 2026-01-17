pub enum SimulationCommand {
    Clear,
    Shutdown,
    Inspect { x: u16, y: u16 },
    SpawnAnt { x: u16, y: u16, tribe: u8 },
    SpawnNest { x: u16, y: u16, tribe: u8 },
    SpawnFood { x: u16, y: u16, amount: u8 },
}
