#[derive(Debug, Default)]
pub struct SimulationStats {
    pub ant_count: u16,
    pub ants_with_food: u16,
    pub total_food: u64,
    pub avg_step_duration_secs: f32,
}
