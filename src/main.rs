mod utils;
use crate::utils::*;
use macroquad::prelude::*;

#[macroquad::main("road_insertion")]
async fn main() {
    let mut vehicles_up: Vec<Vehicle> = vec![];
    let mut vehicles_down: Vec<Vehicle> = vec![];
    let mut vehicles_left: Vec<Vehicle> = vec![];
    let mut vehicles_right: Vec<Vehicle> = vec![];
    request_new_screen_size(800., 800.);
    loop {
        clear_background(BLACK);
        draw_rectangle(350., 0., 100., 800., GRAY);
        draw_rectangle(0., 350., 800., 100., GRAY);
        draw_dashed_line_x(0, 350, 400);
        draw_dashed_line_x(450, 800, 400);
        draw_dashed_line_y(400, 0, 350);
        draw_dashed_line_y(400, 450, 800);

        if is_key_down(KeyCode::Up) {
            if vehicles_up.len() == 0 || vehicles_up[vehicles_up.len() - 1].y <= 710 {
                vehicles_up.push(Vehicle::new(405, 800, 3, Direction::Up));
            }
        } else if is_key_down(KeyCode::Down) {
            if vehicles_down.len() == 0 || vehicles_down[vehicles_down.len() - 1].y >= 90 {
                vehicles_down.push(Vehicle::new(355, 0, 3, Direction::Down));
            }
        } else if is_key_down(KeyCode::Left) {
            if vehicles_left.len() == 0 || vehicles_left[vehicles_left.len() - 1].x <= 710 {
                vehicles_left.push(Vehicle::new(800, 355, 3, Direction::Left));
            }
        } else if is_key_down(KeyCode::Right) {
            if vehicles_right.len() == 0 || vehicles_right[vehicles_right.len() - 1].x >= 90 {
                vehicles_right.push(Vehicle::new(0, 405, 3, Direction::Right));
            }
        }

        for car in vehicles_up.iter_mut() {
            car.update();
            car.draw();
        }
        for car in vehicles_down.iter_mut() {
            car.update();
            car.draw();
        }
        for car in vehicles_left.iter_mut() {
            car.update();
            car.draw();
        }
        for car in vehicles_right.iter_mut() {
            car.update();
            car.draw();
        }

        vehicles_up.retain(|c| c.y > -40);
        vehicles_down.retain(|c| c.y < 800);
        vehicles_left.retain(|c| c.x > -40);
        vehicles_right.retain(|c| c.x < 800);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
