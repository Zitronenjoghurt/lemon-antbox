pub enum SimulationCommand {
    Clear,
    Shutdown,
    SpawnAnt { x: u16, y: u16, tribe: u8 },
    SpawnNest { x: u16, y: u16, tribe: u8 },
    SpawnFood { x: u16, y: u16, amount: u8 },
}
