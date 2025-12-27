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
    shared: Arc<SharedState>,
    thread: JoinHandle<()>,
}

impl ThreadedSimulation {
    pub fn spawn(height: u32, width: u32) -> Self {
        let (command_tx, command_rx) = std::sync::mpsc::channel();

        let shared = Arc::new(SharedState::default());
        let shared_clone = shared.clone();

        let buf_size = (height * width * 4) as usize;
        let (frame_writer, frame_reader) = TripleBuffer::new(&vec![0u8; buf_size]).split();

        let thread = thread::spawn(move || {
            let context = ThreadedContext {
                simulation: Simulation::new(height, width),
                command_rx,
                shared: shared_clone,
                frame_writer,
            };
            context.run();
        });

        Self {
            command_tx,
            frame_reader,
            shared,
            thread,
        }
    }

    pub fn send_command(&self, command: command::SimulationCommand) {
        let _ = self.command_tx.send(command);
    }

    pub fn draw(&mut self, frame: &mut [u8]) {
        let buffer = self.frame_reader.read();
        frame.copy_from_slice(buffer);
    }

    pub fn is_paused(&self) -> bool {
        self.shared.is_paused()
    }

    pub fn set_paused(&self, paused: bool) {
        self.shared.set_paused(paused)
    }

    pub fn toggle_paused(&self) {
        self.shared.set_paused(!self.shared.is_paused());
    }
}

impl Drop for ThreadedSimulation {
    fn drop(&mut self) {
        self.send_command(command::SimulationCommand::Shutdown);
    }
}
