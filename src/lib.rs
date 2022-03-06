use std::io;

use rand::Rng;
use sdl2::{render::Canvas, video::Window};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cell {
    Dead,
    Alive,
}

impl From<bool> for Cell {
    fn from(b: bool) -> Self {
        if b {
            Cell::Alive
        } else {
            Cell::Dead
        }
    }
}

pub struct IndicesEnumerate {
    width: usize,
    height: usize,
    x: usize,
    y: usize,
}

impl Iterator for IndicesEnumerate {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let result = (self.x, self.y);

        self.x += 1;

        if self.x == self.width {
            self.x = 0;
            self.y += 1;
        }

        if self.y == self.height {
            None
        } else {
            Some(result)
        }
    }
}

pub struct Life {
    width: usize,
    height: usize,
    data: Vec<Cell>,
}

impl Life {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![Cell::Dead; width * height],
        }
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        for (x, y) in self.enumerate_indices() {
            self[(x, y)] = rng.gen_bool(0.25).into();
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for (x, y) in self.enumerate_indices() {
            if self[(x, y)] == Cell::Alive {
                canvas.draw_point(sdl2::rect::Point::new(x as i32, y as i32))?;
            }
        }
        Ok(())
    }

    pub fn next(&self) -> Life {
        let mut result = Life::new(self.width, self.height);
        for (x, y) in self.enumerate_indices() {
            let neigbours = self.get_neigbours(x, y);
            match self[(x, y)] {
                Cell::Dead if neigbours == 3 => result[(x, y)] = Cell::Alive,
                Cell::Alive if neigbours >= 2 && neigbours <= 3 => result[(x, y)] = Cell::Alive,
                _ => result[(x, y)] = Cell::Dead,
            };
        }
        result
    }

    pub fn read_from<R: io::Read>(&mut self, mut file: R) -> Result<(), io::Error> {
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        for i in self.enumerate_indices() {
            self[i] = Cell::Dead;
        }

        for (y, line) in contents.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if !c.is_whitespace() && c != '|' {
                    self[(x, y)] = Cell::Alive;
                }
            }
        }

        Ok(())
    }

    fn get_neigbours(&self, x: usize, y: usize) -> usize {
        let x = x as isize;
        let y = y as isize;

        let mut count = 0;

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let x = ((x + i + self.width as isize) % self.width as isize) as usize;
                let y = ((y + j + self.height as isize) % self.height as isize) as usize;

                if self[(x, y)] == Cell::Alive {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn enumerate_indices(&self) -> IndicesEnumerate {
        IndicesEnumerate {
            width: self.width,
            height: self.height,
            x: 0,
            y: 0,
        }
    }
}

impl std::ops::Index<(usize, usize)> for Life {
    type Output = Cell;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 + index.1 * self.width]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Life {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 + index.1 * self.width]
    }
}
