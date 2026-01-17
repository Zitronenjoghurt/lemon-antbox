use crate::simulation::Simulation;
use crate::threaded::ant_buffer::AntBuffer;
use crate::threaded::command::SimulationCommand;
use crate::threaded::event::{InspectedCell, SimulationEvent};
use crate::threaded::shared::SharedState;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::time::Duration;

pub struct ThreadedContext {
    pub simulation: Simulation,
    pub command_rx: Receiver<SimulationCommand>,
    pub event_tx: Sender<SimulationEvent>,
    pub shared: Arc<SharedState>,
    pub frame_writer: triple_buffer::Input<Vec<u8>>,
    pub ant_writer: triple_buffer::Input<Option<AntBuffer>>,
}

impl ThreadedContext {
    pub fn run(mut self) {
        let mut running = true;

        while running {
            let frame_start = std::time::Instant::now();

            self.sync_state();

            while let Ok(command) = self.command_rx.try_recv() {
                if !self.handle_command(command) {
                    running = false;
                }
            }

            self.simulation.step();
            self.sync_frame();
            self.sync_ant();

            let elapsed = frame_start.elapsed();
            let target = Duration::from_secs_f64(1.0 / self.shared.steps_per_second() as f64);

            if let Some(remaining) = target.checked_sub(elapsed) {
                std::thread::sleep(remaining);
            }
        }
    }

    fn sync_state(&mut self) {
        self.shared.sync_settings(self.simulation.settings_mut());
        self.shared.sync_stats(self.simulation.stats());
    }

    fn sync_frame(&mut self) {
        self.simulation.draw(self.frame_writer.input_buffer_mut());
        self.frame_writer.publish();
    }

    fn sync_ant(&mut self) {
        if let Some(index) = self.simulation.settings().inspected_ant
            && let Some(ant) = self.simulation.get_ant(index as usize)
        {
            let senses = self.simulation.sense_for_ant(&ant);
            let ant_buffer = AntBuffer { ant, senses };
            *self.ant_writer.input_buffer_mut() = Some(ant_buffer);
            self.ant_writer.publish();
        }
    }

    pub fn handle_command(&mut self, command: SimulationCommand) -> bool {
        let mut do_continue = true;
        match command {
            SimulationCommand::Clear => self.simulation.clear(),
            SimulationCommand::Inspect { x, y } => self.inspect(x, y),
            SimulationCommand::Shutdown => do_continue = false,
            SimulationCommand::SpawnAnt { x, y, tribe } => self.simulation.spawn_ant(x, y, tribe),
            SimulationCommand::SpawnNest { x, y, tribe } => self.simulation.spawn_nest(x, y, tribe),
            SimulationCommand::SpawnFood { x, y, amount } => {
                self.simulation.spawn_food(x, y, amount)
            }
        }
        do_continue
    }

    fn inspect(&mut self, x: u16, y: u16) {
        let cell = self.simulation.get_cell(x, y);
        let inspected_cell = InspectedCell { x, y, cell };
        self.send_event(SimulationEvent::InspectedCell(Box::new(inspected_cell)));

        if let Some(ant_index) = self.simulation.get_ant_index_at_coords(x, y, 2.0) {
            self.simulation.settings_mut().inspected_ant = Some(ant_index as u16);
        }
    }

    fn send_event(&self, event: SimulationEvent) {
        let _ = self.event_tx.send(event);
    }
}
