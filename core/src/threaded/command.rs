pub enum SimulationCommand {
    Clear,
    Shutdown,
    SpawnAnt((u16, u16)),
}
