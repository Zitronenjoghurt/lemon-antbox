use crate::simulation::Simulation;
use crate::threaded::command::SimulationCommand;
use crate::threaded::shared::SharedState;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::time::Duration;

pub struct ThreadedContext {
    pub simulation: Simulation,
    pub command_rx: Receiver<SimulationCommand>,
    pub shared: Arc<SharedState>,
    pub frame_writer: triple_buffer::Input<Vec<u8>>,
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

            let elapsed = frame_start.elapsed();
            let target = Duration::from_secs_f64(1.0 / self.shared.steps_per_second() as f64);

            if let Some(remaining) = target.checked_sub(elapsed) {
                std::thread::sleep(remaining);
            }
        }
    }

    fn sync_state(&mut self) {
        self.simulation.set_paused(self.shared.is_paused());
        self.shared.set_ant_count(self.simulation.ant_count());
    }

    fn sync_frame(&mut self) {
        self.simulation.draw(self.frame_writer.input_buffer_mut());
        self.frame_writer.publish();
    }

    pub fn handle_command(&mut self, command: SimulationCommand) -> bool {
        let mut do_continue = true;
        match command {
            SimulationCommand::Clear => self.simulation.clear(),
            SimulationCommand::Shutdown => do_continue = false,
            SimulationCommand::SpawnAnt((x, y)) => self.simulation.spawn_ant(x, y),
        }
        do_continue
    }
}
