use macroquad::prelude::*;

#[macroquad::main("My game!")]
async fn main() {
    loop {
        clear_background(GREEN);
        next_frame().await
    }
}