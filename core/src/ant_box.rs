use crate::ant_box::ant::Ant;
use crate::ant_box::cell::Cell;

mod ant;
mod cell;

#[derive(Debug)]
pub struct AntBox {
    ants: Vec<Ant>,
    cells: Vec<Cell>,
    height: u16,
    width: u16,
}

impl AntBox {
    pub fn new(height: u16, width: u16) -> Self {
        let cells = vec![Cell::default(); (height as usize * width as usize)];
        Self {
            ants: Vec::new(),
            cells,
            height,
            width,
        }
    }

    pub fn draw(&self, frame: &mut [u8]) {
        self.draw_cells(frame);
        self.draw_ants(frame);
    }

    fn draw_ants(&self, frame: &mut [u8]) {
        for ant in &self.ants {
            let index = self.coords_to_index(ant.x, ant.y) * 4;
            frame[index..index + 4].copy_from_slice(&ant.color_rgba());
        }
    }

    fn draw_cells(&self, frame: &mut [u8]) {
        for (cell, pixel) in self.cells.iter().zip(frame.chunks_exact_mut(4)) {
            pixel.copy_from_slice(&cell.color_rgba());
        }
    }

    pub fn clear(&mut self) {
        self.ants.clear();
        self.cells = vec![Cell::default(); (self.height as usize * self.width as usize)];
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }

    pub fn get_width(&self) -> u16 {
        self.width
    }

    pub fn ant_count(&self) -> u16 {
        self.ants.len() as u16
    }

    fn coords_to_index(&self, x: u16, y: u16) -> usize {
        y as usize * self.width as usize + x as usize
    }

    pub fn spawn_ant(&mut self, x: u16, y: u16) {
        if x >= self.width || y >= self.height || self.ants.len() > 65535 {
            return;
        }

        let ant = Ant { x, y };
        self.ants.push(ant);
    }
}
