use macroquad::prelude::*;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}

fn move_circle(circle: Shape, delta_time: f32) -> Shape {
    let mut new_circle = Shape {
        size: circle.size,
        speed: circle.speed,
        x: circle.x,
        y: circle.y,
    };
    if is_key_down(KeyCode::Right) {
        new_circle.x += new_circle.speed * delta_time;
    }
    if is_key_down(KeyCode::Left) {
        new_circle.x -= new_circle.speed * delta_time;
    }
    if is_key_down(KeyCode::Down) {
        new_circle.y += new_circle.speed * delta_time;
    }
    if is_key_down(KeyCode::Up) {
        new_circle.y -= new_circle.speed * delta_time;
    }

    new_circle.x = clamp(new_circle.x, 0.0, screen_width());
    new_circle.y = clamp(new_circle.y, 0.0, screen_height());
    return new_circle
}

fn draw_squares(squares: &[Shape]) {
    for square in squares {
        draw_rectangle(
        square.x - square.size / 2.0,
        square.y - square.size / 2.0,
        square.size,
        square.size,
        GREEN,
        );
    }
}

#[macroquad::main("My game!")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);
    let mut squares: Vec<Shape> = vec![];
    const MOVEMENT_SPEED: f32 = 200.0;
    let mut circle = Shape {
        size: 16.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };
    loop {
        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size,
            });
        }
        let delta_time = get_frame_time();
        clear_background(SKYBLUE);

        circle = move_circle(circle, delta_time);
        draw_circle(circle.x, circle.y, circle.size, DARKBLUE);

        for square in &mut squares {
            square.y += square.speed * delta_time;
        }            
        squares.retain(|square| square.y < screen_height() + square.size);
        draw_squares(&squares);

        next_frame().await
    }
}
    