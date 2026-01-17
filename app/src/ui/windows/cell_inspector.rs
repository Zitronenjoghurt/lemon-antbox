use crate::ui::widgets::simulation_ant::SimulationAnt;
use crate::ui::widgets::simulation_cell::SimulationCell;
use crate::ui::windows::{ToggleableUiWindow, UiWindow};
use egui::{Id, Ui, Widget, WidgetText};
use lemon_antbox_core::threaded::event::InspectedCell;
use lemon_antbox_core::threaded::ThreadedSimulation;

#[derive(Default)]
pub struct CellInspectorWindowState {
    pub is_open: bool,
    inspected_cell: InspectedCell,
    has_inspected: bool,
}

impl CellInspectorWindowState {
    pub fn inspect(&mut self, cell: InspectedCell) {
        self.inspected_cell = cell;
        self.has_inspected = true;
    }
}

pub struct CellInspectorWindow<'a> {
    state: &'a mut CellInspectorWindowState,
    sim: &'a mut ThreadedSimulation,
}

impl<'a> CellInspectorWindow<'a> {
    pub fn new(state: &'a mut CellInspectorWindowState, sim: &'a mut ThreadedSimulation) -> Self {
        Self { state, sim }
    }
}

impl UiWindow for CellInspectorWindow<'_> {
    fn id() -> Id {
        Id::new("cell_inspector_window")
    }

    fn title() -> impl Into<WidgetText> {
        "Cell Inspector"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        if !self.state.has_inspected {
            ui.small("Right-click on a cell to inspect it.");
            return;
        }

        ui.vertical(|ui| {
            if let Some(cell) = self.state.inspected_cell.cell {
                let coords = (self.state.inspected_cell.x, self.state.inspected_cell.y);
                SimulationCell::new(coords, &cell).ui(ui);
            }

            if let Some(buffer) = self.sim.inspected_ant() {
                ui.separator();
                SimulationAnt::new(&buffer.ant, &buffer.senses, 0).ui(ui);
            }
        });
    }
}

impl ToggleableUiWindow for CellInspectorWindow<'_> {
    fn toggle_label(&self) -> String {
        egui_phosphor::regular::MAGNIFYING_GLASS.into()
    }
}
