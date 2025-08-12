use macroquad::prelude::*;

#[derive(Clone)]
pub struct Vehicle {
    pub x: u32,
    pub y: u32,
    pub speed: u32,
    pub color: Color,
    pub direction: Direction,
}

#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Vehicle {
    pub fn new(x: u32, y: u32, speed: u32, color: Color, direction: Direction) -> Self {
        Vehicle {
            x,
            y,
            speed,
            color,
            direction,
        }
    }

    pub fn update(&mut self) {
        match self.direction {
            Direction::Up => {
                if self.y <= self.speed {
                    self.y = 0;
                } else {
                    self.y -= self.speed
                }
            }
            Direction::Down => {
                if self.y + self.speed >= 800 {
                    self.y = 800;
                } else {
                    self.y += self.speed
                }
            }
            Direction::Left => {
                if self.x <= self.speed {
                    self.x = 0;
                } else {
                    self.x -= self.speed
                }
            }
            Direction::Right => {
                if self.x + self.speed >= 800 {
                    self.x = 800;
                } else {
                    self.x += self.speed
                }
            }
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.x as f32, self.y as f32, 40.0, 40.0, self.color);
    }
}

impl Direction {
    pub fn random() -> Self {
        match quad_rand::gen_range(0, 4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => unreachable!(),
        }
    }
}

pub fn draw_dashed_line_x(x1: u32, x2: u32, y: u32) {
    let mut x: u32 = x1;
    loop {
        draw_line(x as f32, y as f32, x as f32 + 50., y as f32, 3., WHITE);
        x += 60;
        if x >= x2 {
            break;
        }
    }
}

pub fn draw_dashed_line_y(x: u32, y1: u32, y2: u32) {
    let mut y: u32 = y1;
    loop {
        draw_line(x as f32, y as f32, x as f32, y as f32 + 50., 3., WHITE);
        y += 60;
        if y >= y2 {
            break;
        }
    }
}
