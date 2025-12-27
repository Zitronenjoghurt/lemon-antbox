use crate::ui::windows::{ToggleableUiWindow, UiWindow};
use egui::{Id, Ui, Widget, WidgetText};
use lemon_antbox_core::threaded::ThreadedSimulation;

pub struct SimulationSettingsWindow<'a> {
    is_open: &'a mut bool,
}

impl<'a> SimulationSettingsWindow<'a> {
    pub fn new(is_open: &'a mut bool) -> Self {
        Self { is_open }
    }
}

impl UiWindow for SimulationSettingsWindow<'_> {
    fn id() -> Id {
        Id::new("simulation_settings_window")
    }

    fn title() -> impl Into<WidgetText> {
        "Simulation Settings"
    }

    fn is_open(&self) -> bool {
        *self.is_open
    }

    fn set_open(&mut self, open: bool) {
        *self.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui, sim: &ThreadedSimulation) {
        ui.horizontal(|ui| {
            let mut paused = sim.state().is_paused();
            egui::Checkbox::new(&mut paused, "Paused").ui(ui);
            sim.state().set_paused(paused);

            if ui.button("Clear").clicked() {
                sim.clear();
            }
        });
    }
}

impl ToggleableUiWindow for SimulationSettingsWindow<'_> {
    fn toggle_label(&self) -> String {
        egui_phosphor::regular::GEAR.to_string()
    }
}
