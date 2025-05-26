use macroquad::prelude::*;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
        
    }
    fn rect(&self) -> Rect {
        Rect {
        x: self.x - self.size / 2.0,
        y: self.y - self.size / 2.0,
        w: self.size,
        h: self.size,
        }
    }
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
    let mut gameover = false;
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
        clear_background(SKYBLUE);
        let delta_time = get_frame_time();
        if !gameover {
            if rand::gen_range(0, 99) >= 95 {
                let size = rand::gen_range(16.0, 64.0);
                squares.push(Shape {
                    size,
                    speed: rand::gen_range(50.0, 150.0),
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                });
            }
    
            circle = move_circle(circle, delta_time);
            for square in &mut squares {
                square.y += square.speed * delta_time;
            } 
        }
        
        draw_circle(circle.x, circle.y, circle.size, DARKBLUE);           
        squares.retain(|square| square.y < screen_height() + square.size);
        draw_squares(&squares);
        
        if squares.iter().any(|square| circle.collides_with(square)) {
            gameover = true;
        }
        
        if gameover {
            let text = "GAME OVER!";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0,
                50.0,
                RED,
            );
        }                

        if gameover && is_key_pressed(KeyCode::Space) {
            squares.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            gameover = false;
        }
            

        next_frame().await
    }
}
    