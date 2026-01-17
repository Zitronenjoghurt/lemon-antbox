use crate::ui::windows::{ToggleableUiWindow, UiWindow};
use egui::{Grid, Id, Ui, WidgetText};
use lemon_antbox_core::threaded::ThreadedSimulation;

pub struct SimulationStatsWindow<'a> {
    pub is_open: &'a mut bool,
    sim: &'a ThreadedSimulation,
}

impl<'a> SimulationStatsWindow<'a> {
    pub fn new(is_open: &'a mut bool, sim: &'a ThreadedSimulation) -> Self {
        Self { is_open, sim }
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

    fn render_content(&mut self, ui: &mut Ui) {
        Grid::new("simulation_stats_grid")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.label("Ant Count");
                ui.label(self.sim.state().ant_count().to_string());
                ui.end_row();

                ui.label("Ants with Food");
                ui.label(self.sim.state().ants_with_food().to_string());
                ui.end_row();

                ui.label("Total Food");
                ui.label(self.sim.state().total_food().to_string());
                ui.end_row();

                let avg_step_duration_secs = self.sim.state().avg_step_duration_secs();
                ui.label("Avg. Step Duration");
                ui.label(format!("{:.02}ms", avg_step_duration_secs * 1000.0));
                ui.end_row();
            });
    }
}

impl ToggleableUiWindow for SimulationStatsWindow<'_> {
    fn toggle_label(&self) -> String {
        egui_phosphor::regular::CHART_BAR_HORIZONTAL.into()
    }
}
