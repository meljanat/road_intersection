use macroquad::prelude::*;

#[derive(Clone)]
pub struct Vehicle {
    pub x: i32,
    pub y: i32,
    pub speed: i32,
    pub color: Color,
    pub direction: Direction,
    pub turn: Turn,
    pub dar: bool,
    pub flmorb3 :bool,
    pub pause : bool
}

#[derive(Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq)]
pub enum Turn {
    Left,
    Right,
    Straight,
}

const SCARLET: Color = Color::from_rgba(255, 36, 0, 255);

impl Vehicle {
    pub fn new(x: i32, y: i32, speed: i32, direction: Direction) -> Self {
        let mut a = Vehicle {
            x,
            y,
            speed,
            color: WHITE,
            direction,
            turn: Turn::Straight,
            dar : false,
            flmorb3 : false ,
            pause : false ,
        };
            // a.color = YELLOW;
        if a.direction == Direction::Right {
            a.color = BLUE;
        } else if a.direction == Direction::Left {
            a.color = YELLOW;
        } else if a.direction == Direction::Up {
            a.color = RED;
        } else {
            a.color = YELLOW;
        }
        a.turn = match a.color {
            SCARLET => Turn::Straight,
            BLUE => Turn::Left,
            YELLOW => Turn::Right,
            _ => Turn::Straight,
        };
        a
    }

    pub fn color(&self) -> Color {
        let colors = vec![SCARLET, BLUE, YELLOW];
        colors[quad_rand::gen_range(0, colors.len())]
    }

    pub fn update(&mut self) {
        if is_key_down(KeyCode::Space) {
            self.pause = false;
        } 
        if self.pause {
            return;
        }
        match self.direction {
            
            Direction::Up => {
                self.y -= self.speed;
                if self.y <= 450 {
                    self.pause = true;
                }
                if self.y <= 405 && !self.dar {
                    self.flmorb3 = true;
                    if self.turn == Turn::Right{
                        self.direction = Direction::Right;
                        self.dar = true;
                    } else if self.turn == Turn::Left && self.y <= 355 {
                        self.direction = Direction::Left;
                        self.dar = true;
                    }
                }
            }
            Direction::Down => {
                self.y += self.speed;   
                if self.y >= 320 {
                    self.pause = true;
                }
                if self.y >= 355 && !self.dar {
                    self.flmorb3 = true;
                    if self.turn == Turn::Right  {
                        self.direction = Direction::Left;
                        self.dar = true;
                    } else if self.turn == Turn::Left && self.y >= 405 {
                        self.direction = Direction::Right;
                        self.dar = true;
                    }
                }
            }
            Direction::Left => {
                self.x -= self.speed;
                if self.x <= 450 {
                    self.pause = true;
                }
                if self.x <= 405 && !self.dar {
                    self.flmorb3 = true;
                    if self.turn == Turn::Right {
                        self.direction = Direction::Up;
                        self.dar = true;
                    } else if self.turn == Turn::Left && self.x <= 355 {
                        self.direction = Direction::Down;
                        self.dar = true;
                    }
                }
            }
            Direction::Right => {
                self.x += self.speed;
                if self.x >= 320 {
                    self.pause = true;
                }
                if self.x >= 355 && !self.dar {
                    self.flmorb3 = true;
                    if self.turn == Turn::Right {
                        self.direction = Direction::Down;
                        self.dar = true;
                    } else if self.turn == Turn::Left && self.x >= 405 {
                        self.direction = Direction::Up;
                        self.dar = true;
                    }
                }
            }
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.x as f32, self.y as f32, 30.0, 30.0, self.color);
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
