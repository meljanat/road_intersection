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
    pub flmorb3: bool,
    pub pause: bool,
}

pub struct TrafficLights {
    pub up: TrafficLight,
    pub down: TrafficLight,
    pub right: TrafficLight,
    pub left: TrafficLight,
}

pub struct TrafficLight {
    pub x: i32,
    pub y: i32,
    pub light: Light,
}

#[derive(Clone, PartialEq)]
pub enum Light {
    Red,
    Green,
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
            color: Vehicle::color(),
            direction,
            turn: Turn::Straight,
            dar: false,
            flmorb3: false,
            pause: false,
        };
        a.turn = match a.color {
            SCARLET => Turn::Straight,
            BLUE => Turn::Left,
            YELLOW => Turn::Right,
            _ => Turn::Straight,
        };
        a
    }

    pub fn color() -> Color {
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
                if self.y <= 451 {
                    self.pause = true;
                }
                if self.y <= 405 && !self.dar {
                    self.flmorb3 = true;
                    if self.turn == Turn::Right {
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
                if self.y >= 301 {
                    self.pause = true;
                }
                if self.y >= 355 && !self.dar {
                    self.flmorb3 = true;
                    if self.turn == Turn::Right {
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
                if self.x >= 300 {
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

impl TrafficLights {
    pub fn draw(&self) {
        match self.up.light {
            Light::Red => draw_rectangle(self.up.x as f32, self.up.y as f32, 20., 20., RED),
            Light::Green => draw_rectangle(self.up.x as f32, self.up.y as f32, 20., 20., GREEN),
        }
        match self.down.light {
            Light::Red => draw_rectangle(self.down.x as f32, self.down.y as f32, 20., 20., RED),
            Light::Green => draw_rectangle(self.down.x as f32, self.down.y as f32, 20., 20., GREEN),
        }
        match self.left.light {
            Light::Red => draw_rectangle(self.left.x as f32, self.left.y as f32, 20., 20., RED),
            Light::Green => draw_rectangle(self.left.x as f32, self.left.y as f32, 20., 20., GREEN),
        }
        match self.right.light {
            Light::Red => draw_rectangle(self.right.x as f32, self.right.y as f32, 20., 20., RED),
            Light::Green => draw_rectangle(self.right.x as f32, self.right.y as f32, 20., 20., GREEN),
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

pub fn make_lights() -> TrafficLights {
    TrafficLights {
        up: TrafficLight {
            x: 330,
            y: 330,
            light: Light::Red,
        },
        down: TrafficLight {
            x: 450,
            y: 450,
            light: Light::Green,
        },
        left: TrafficLight {
            x: 330,
            y: 450,
            light: Light::Red,
        },
        right: TrafficLight {
            x: 450,
            y: 330,
            light: Light::Green,
        },
    }
}
