use crate::simulation::cell::Cell;

pub enum SimulationEvent {
    InspectedCell(Box<InspectedCell>),
}

#[derive(Debug, Default)]
pub struct InspectedCell {
    pub x: u16,
    pub y: u16,
    pub cell: Option<Cell>,
}
