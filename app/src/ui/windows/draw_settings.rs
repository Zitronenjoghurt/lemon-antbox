use crate::ui::types::draw_mode::DrawMode;
use crate::ui::widgets::enum_select::EnumSelect;
use crate::ui::windows::{ToggleableUiWindow, UiWindow};
use egui::{Id, Slider, Ui, Widget, WidgetText};
use lemon_antbox_core::threaded::ThreadedSimulation;

#[derive(Debug, Default)]
pub struct DrawSettingsWindowState {
    pub is_open: bool,
    pub draw_mode: DrawMode,
    pub ant_tribe: u8,
    pub nest_tribe: u8,
    pub food_amount: u8,
}

pub struct DrawSettingsWindow<'a> {
    state: &'a mut DrawSettingsWindowState,
}

impl<'a> DrawSettingsWindow<'a> {
    pub fn new(state: &'a mut DrawSettingsWindowState) -> Self {
        Self { state }
    }
}

impl UiWindow for DrawSettingsWindow<'_> {
    fn id() -> Id {
        Id::new("draw_settings_window")
    }

    fn title() -> impl Into<WidgetText> {
        "Draw Settings"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui, sim: &ThreadedSimulation) {
        ui.vertical(|ui| {
            EnumSelect::new(&mut self.state.draw_mode, "draw_settings_draw_mode")
                .label("Draw Mode")
                .ui(ui);

            let max_tribe = sim.state().tribe_count().saturating_sub(1);
            match self.state.draw_mode {
                DrawMode::Ant => {
                    ui.horizontal(|ui| {
                        ui.label("Tribe");
                        ui.add(Slider::new(&mut self.state.ant_tribe, 0..=max_tribe));
                    });
                }
                DrawMode::Nest => {
                    ui.horizontal(|ui| {
                        ui.label("Tribe");
                        ui.add(Slider::new(&mut self.state.nest_tribe, 0..=max_tribe));
                    });
                }
                DrawMode::Food => {
                    ui.horizontal(|ui| {
                        ui.label("Amount");
                        ui.add(Slider::new(&mut self.state.food_amount, 1..=255));
                    });
                }
            }
        });
    }
}

impl ToggleableUiWindow for DrawSettingsWindow<'_> {
    fn toggle_label(&self) -> String {
        egui_phosphor::regular::PAINT_BRUSH.into()
    }
}
