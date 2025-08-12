mod utils;
use crate::utils::*;
use macroquad::prelude::*;

#[macroquad::main("road_insertion")]
async fn main() {
    let mut vehicles: Vec<Vehicle> = vec![];
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
            vehicles.push(Vehicle::new(405, 800, 3, GREEN, Direction::Up));
        } else if is_key_down(KeyCode::Down) {
            vehicles.push(Vehicle::new(405, 0, 3, RED, Direction::Down));
        } else if is_key_down(KeyCode::Left) {
            vehicles.push(Vehicle::new(800, 405, 3, BLUE, Direction::Left));
        } else if is_key_down(KeyCode::Right) {
            vehicles.push(Vehicle::new(0, 405, 3, YELLOW, Direction::Right));
        }

        vehicles.iter().for_each(Vehicle::draw);
        vehicles.iter_mut().for_each(Vehicle::update);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
