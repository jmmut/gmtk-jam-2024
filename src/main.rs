use macroquad::prelude::*;

type Pixels = f32;

const PAD: Pixels = 20.0;
const EDITOR_SIZE: Pixels = 100.0;
const THICKNESS: f32 = 2.0;
const RADIUS: f32 = 10.0;

const FAINT_CIRCLE_COLOR: Color = Color::new(0.8, 0.8, 0.2, 0.2);
const STRONG_CIRCLE_COLOR: Color = Color::new(0.8, 0.8, 0.2, 0.7);
const SETTLED_CIRCLE_COLOR: Color = Color::new(0.8, 0.5, 0.2, 0.7);

#[macroquad::main("MY_CRATE_NAME")]
async fn main() {
    let mut circles = Vec::<Vec2>::new();
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(DARKGRAY);
        draw_rectangle_lines(PAD, PAD, EDITOR_SIZE, EDITOR_SIZE, THICKNESS, LIGHTGRAY);

        let mouse_pos = mouse_position();
        if let Some(pos) = pos_in_editor(mouse_pos) {
            let absolute_pos = normalized_to_editor_absolute(pos);
            let color = if is_mouse_button_down(MouseButton::Left) {
                STRONG_CIRCLE_COLOR
            } else {
                FAINT_CIRCLE_COLOR
            };
            if is_mouse_button_released(MouseButton::Left) {
                circles.push(pos);
            } else {
                draw_circle_lines(absolute_pos.x, absolute_pos.y, 10.0, THICKNESS, color);
            }
        }
        for circle in &circles {
            let absolute_pos = normalized_to_editor_absolute(*circle);
            draw_circle_lines(
                absolute_pos.x,
                absolute_pos.y,
                RADIUS,
                THICKNESS,
                SETTLED_CIRCLE_COLOR,
            );

            let absolute_pos = normalized_to_canvas_absolute(*circle);
            draw_circle(absolute_pos.x, absolute_pos.y, RADIUS, SETTLED_CIRCLE_COLOR);
        }

        next_frame().await
    }
}

fn pos_in_editor((x, y): (f32, f32)) -> Option<Vec2> {
    return if x >= PAD && x < PAD + EDITOR_SIZE && y >= PAD && y < PAD + EDITOR_SIZE {
        Some(editor_absolute_to_normalized(Vec2::new(x, y)))
    } else {
        None
    };
}

fn editor_absolute_to_normalized(pos: Vec2) -> Vec2 {
    return Vec2::new((pos.x - PAD) / EDITOR_SIZE, (pos.y - PAD) / EDITOR_SIZE);
}
fn normalized_to_editor_absolute(pos: Vec2) -> Vec2 {
    return Vec2::new(pos.x * EDITOR_SIZE + PAD, pos.y * EDITOR_SIZE + PAD);
}
fn normalized_to_canvas_absolute(pos: Vec2) -> Vec2 {
    let canvas_x = screen_width() - 4.0 * PAD - EDITOR_SIZE;
    let canvas_y = screen_height() - 2.0 * PAD;
    return Vec2::new(
        pos.x * canvas_x + 3.0 * PAD + EDITOR_SIZE,
        pos.y * canvas_y + PAD,
    );
}
