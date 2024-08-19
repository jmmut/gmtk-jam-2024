use macroquad::prelude::*;

type Pixels = f32;

const PAD: Pixels = 10.0;
const EDITOR_SIZE: Pixels = 100.0;
const THICKNESS: f32 = 2.0;

const FAINT_CIRCLE_COLOR: Color = Color::new(0.8, 0.8, 0.2, 0.2);
const STRONG_CIRCLE_COLOR: Color = Color::new(0.8, 0.8, 0.2, 0.7);

#[macroquad::main("MY_CRATE_NAME")]
async fn main() {
    loop {
        clear_background(DARKGRAY);
        draw_rectangle_lines(PAD, PAD, EDITOR_SIZE, EDITOR_SIZE, THICKNESS, LIGHTGRAY);

        let color = if is_mouse_button_down(MouseButton::Left) {
            STRONG_CIRCLE_COLOR
        } else {
            FAINT_CIRCLE_COLOR
        };

        let mouse_pos = mouse_position();
        if let Some(pos) = pos_in_editor(mouse_pos) {
            let absolute_pos = editor_normalized_to_absolute(pos);
            draw_circle_lines(absolute_pos.x, absolute_pos.y, 10.0, 2.0, color);
        }

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        //
        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await
    }
}

fn pos_in_editor((x, y): (f32, f32)) -> Option<Vec2> {
    return if x >= PAD && x < PAD + EDITOR_SIZE && y >= PAD && y < PAD + EDITOR_SIZE {
        Some(absolute_to_editor_normalized(Vec2::new(x, y)))
    } else {
        None
    };
}

fn editor_normalized_to_absolute(pos: Vec2) -> Vec2 {
    return Vec2::new(pos.x * EDITOR_SIZE + PAD, pos.y * EDITOR_SIZE + PAD);
}
fn absolute_to_editor_normalized(pos: Vec2) -> Vec2 {
    return Vec2::new((pos.x - PAD) / EDITOR_SIZE, (pos.y - PAD) / EDITOR_SIZE);
}
