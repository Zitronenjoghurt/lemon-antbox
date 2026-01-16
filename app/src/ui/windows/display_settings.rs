use crate::ui::widgets::option_enum_select::OptionEnumSelect;
use crate::ui::windows::{ToggleableUiWindow, UiWindow};
use egui::{Grid, Ui, Widget, WidgetText};
use lemon_antbox_core::threaded::ThreadedSimulation;

pub struct DisplaySettingsWindow<'a> {
    is_open: &'a mut bool,
}

impl<'a> DisplaySettingsWindow<'a> {
    pub fn new(is_open: &'a mut bool) -> Self {
        Self { is_open }
    }
}

impl UiWindow for DisplaySettingsWindow<'_> {
    fn id() -> egui::Id {
        egui::Id::new("display_settings_window")
    }

    fn title() -> impl Into<WidgetText> {
        "Display Settings"
    }

    fn is_open(&self) -> bool {
        *self.is_open
    }

    fn set_open(&mut self, open: bool) {
        *self.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui, sim: &ThreadedSimulation) {
        Grid::new("display_settings_grid")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.label("Drawn Pheromone");
                let mut drawn_pheromone = sim.state().drawn_pheromone();
                OptionEnumSelect::new(&mut drawn_pheromone, "Drawn Pheromone").ui(ui);
                sim.state().set_drawn_pheromone(drawn_pheromone);
                ui.end_row();
            });
    }
}

impl ToggleableUiWindow for DisplaySettingsWindow<'_> {
    fn toggle_label(&self) -> String {
        egui_phosphor::regular::MONITOR.to_string()
    }
}
