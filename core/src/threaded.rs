use crate::simulation::settings::SimulationSettings;
use crate::simulation::Simulation;
use crate::threaded::command::SimulationCommand;
use crate::threaded::context::ThreadedContext;
use crate::threaded::shared::SharedState;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use triple_buffer::TripleBuffer;

mod command;
mod context;
mod event;
mod shared;

pub struct ThreadedSimulation {
    command_tx: Sender<SimulationCommand>,
    frame_reader: triple_buffer::Output<Vec<u8>>,
    state: Arc<SharedState>,
    _thread: JoinHandle<()>,
}

impl ThreadedSimulation {
    pub fn spawn(settings: SimulationSettings) -> Self {
        let (command_tx, command_rx) = std::sync::mpsc::channel();

        let shared = Arc::new(SharedState::from_settings(&settings));
        let shared_clone = shared.clone();

        let buf_size = settings.cell_count() * 4;
        let (frame_writer, frame_reader) = TripleBuffer::new(&vec![0u8; buf_size]).split();

        let thread = thread::spawn(move || {
            let context = ThreadedContext {
                simulation: Simulation::new(settings),
                command_rx,
                shared: shared_clone,
                frame_writer,
            };
            context.run();
        });

        Self {
            command_tx,
            frame_reader,
            state: shared,
            _thread: thread,
        }
    }

    pub fn send_command(&self, command: SimulationCommand) {
        let _ = self.command_tx.send(command);
    }

    pub fn draw(&mut self, frame: &mut [u8]) {
        let buffer = self.frame_reader.read();
        frame.copy_from_slice(buffer);
    }

    pub fn toggle_paused(&self) {
        self.state.set_paused(!self.state.is_paused());
    }

    pub fn state(&self) -> &SharedState {
        &self.state
    }

    pub fn clear(&self) {
        self.send_command(SimulationCommand::Clear);
    }

    pub fn spawn_ant(&self, x: u16, y: u16, tribe: u8) {
        self.send_command(SimulationCommand::SpawnAnt { x, y, tribe });
    }

    pub fn spawn_nest(&self, x: u16, y: u16, tribe: u8) {
        self.send_command(SimulationCommand::SpawnNest { x, y, tribe });
    }

    pub fn spawn_food(&self, x: u16, y: u16, amount: u8) {
        self.send_command(SimulationCommand::SpawnFood { x, y, amount });
    }
}

impl Drop for ThreadedSimulation {
    fn drop(&mut self) {
        self.send_command(SimulationCommand::Shutdown);
    }
}
