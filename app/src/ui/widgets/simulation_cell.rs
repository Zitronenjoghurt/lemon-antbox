use egui::{Grid, Response, Ui, Widget};
use lemon_antbox_core::simulation::cell::Cell;

pub struct SimulationCell<'a> {
    coords: (u16, u16),
    cell: &'a Cell,
    id: &'a str,
}

impl<'a> SimulationCell<'a> {
    pub fn new(coords: (u16, u16), cell: &'a Cell) -> Self {
        Self {
            coords,
            cell,
            id: "simulation_cell",
        }
    }

    pub fn id(mut self, id: &'a str) -> Self {
        self.id = id;
        self
    }
}

impl Widget for SimulationCell<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        Grid::new(self.id)
            .striped(true)
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Coordinates");
                ui.label(format!("({}, {})", self.coords.0, self.coords.1));
                ui.end_row();

                ui.label("Food");
                ui.label(self.cell.food.to_string());
                ui.end_row();

                ui.label("Has Home");
                if self.cell.flags.has_home() {
                    ui.label(egui_phosphor::regular::CHECK);
                } else {
                    ui.label(egui_phosphor::regular::X);
                }
                ui.end_row();

                if self.cell.flags.has_home() {
                    ui.label("Tribe");
                    ui.label(self.cell.tribe.to_string());
                    ui.end_row();
                }
            })
            .response
    }
}
