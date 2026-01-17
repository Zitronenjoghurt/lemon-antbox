use egui::{Grid, Ui, Widget};
use lemon_antbox_core::simulation::ant::{Ant, AntSenses};

pub struct SimulationAnt<'a> {
    ant: &'a Ant,
    senses: &'a AntSenses,
    index: usize,
}

impl<'a> SimulationAnt<'a> {
    pub fn new(ant: &'a Ant, senses: &'a AntSenses, index: usize) -> Self {
        Self { ant, senses, index }
    }
}

impl Widget for SimulationAnt<'_> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        Grid::new(format!("ant_grid_{}", self.index))
            .striped(true)
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Tribe");
                ui.label(self.ant.tribe.to_string());
                ui.end_row();

                ui.label("Coordinates");
                ui.label(format!("({}, {})", self.ant.x, self.ant.y));
                ui.end_row();

                ui.label("Angle");
                ui.label(format!("{:.2}°", self.ant.angle.to_degrees()));
                ui.end_row();

                ui.label("Has food");
                if self.ant.has_food {
                    ui.label(egui_phosphor::regular::CHECK);
                } else {
                    ui.label(egui_phosphor::regular::X);
                }
                ui.end_row();

                ui.label("At home");
                if self.senses.at_home {
                    ui.label(egui_phosphor::regular::CHECK);
                } else {
                    ui.label(egui_phosphor::regular::X);
                }
                ui.end_row();

                ui.label("Mode");
                ui.label(format!("{:?}", self.ant.mode));
                ui.end_row();

                ui.label("Desired pheromone");
                ui.label(format!("{:?}", self.ant.desired_pheromone()));
                ui.end_row();

                ui.label("Pheromone sensors");
                ui.label(format!(
                    "({:.2} | {:.2} | {:.2})",
                    self.senses.left, self.senses.forward, self.senses.right
                ));
                ui.end_row();

                ui.label("Sensed food");
                ui.label(self.senses.food.to_string());
                ui.end_row();

                ui.label("Spiral radius");
                ui.label(format!("{:.2}", self.ant.spiral_radius));
                ui.end_row();
            })
            .response
    }
}
