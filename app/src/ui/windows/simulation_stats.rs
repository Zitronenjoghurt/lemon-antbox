use crate::ui::windows::{ToggleableUiWindow, UiWindow};
use egui::{Grid, Id, Ui, WidgetText};
use lemon_antbox_core::threaded::ThreadedSimulation;

pub struct SimulationStatsWindow<'a> {
    pub is_open: &'a mut bool,
}

impl<'a> SimulationStatsWindow<'a> {
    pub fn new(is_open: &'a mut bool) -> Self {
        Self { is_open }
    }
}

impl UiWindow for SimulationStatsWindow<'_> {
    fn id() -> Id {
        Id::new("simulation_stats_window")
    }

    fn title() -> impl Into<WidgetText> {
        "Simulation Stats"
    }

    fn is_open(&self) -> bool {
        *self.is_open
    }

    fn set_open(&mut self, open: bool) {
        *self.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui, sim: &ThreadedSimulation) {
        Grid::new("simulation_stats_grid")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.label("Ant Count");
                ui.label(sim.ant_count().to_string());
                ui.end_row();
            });
    }
}

impl ToggleableUiWindow for SimulationStatsWindow<'_> {
    fn toggle_label(&self) -> String {
        egui_phosphor::regular::CHART_BAR_HORIZONTAL.into()
    }
}
