use macroquad::prelude::*;

type Pixels = f32;
type NormalizedPosition = Vec2;
type PixelPosition = Vec2;

const PAD: Pixels = 20.0;
const EDITOR_SIZE: Pixels = 100.0;
const THICKNESS: f32 = 2.0;
const RADIUS: f32 = 10.0;

const FAINT_CIRCLE_COLOR: Color = Color::new(0.8, 0.8, 0.2, 0.2);
const STRONG_CIRCLE_COLOR: Color = Color::new(0.8, 0.8, 0.2, 0.7);
const SETTLED_CIRCLE_COLOR: Color = Color::new(0.8, 0.5, 0.2, 0.7);

#[macroquad::main("MY_CRATE_NAME")]
async fn main() {
    let mut circles = Vec::<NormalizedPosition>::new();
    let mut selected = None;
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(DARKGRAY);
        draw_rectangle_lines(PAD, PAD, EDITOR_SIZE, EDITOR_SIZE, THICKNESS, LIGHTGRAY);

        edit_circles(&mut circles, &mut selected);

        draw_circles(&circles, selected);

        next_frame().await
    }
}

fn edit_circles(mut circles: &mut Vec<NormalizedPosition>, selected: &mut Option<usize>) {
    let mouse_pos = mouse_position();
    if let Some(pos) = pos_in_editor(mouse_pos) {
        let mut absolute_pos = normalized_to_editor_absolute(pos);
        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some(selected_pos) = maybe_take(pos, &mut circles) {
                absolute_pos = normalized_to_editor_absolute(pos);
                *selected = None;
            }
        }
        if is_mouse_button_down(MouseButton::Left) {
            draw_circle_lines(
                absolute_pos.x,
                absolute_pos.y,
                RADIUS,
                THICKNESS,
                STRONG_CIRCLE_COLOR,
            );
        } else if is_mouse_button_released(MouseButton::Left) {
            circles.push(pos);
        } else {
            if let Some(selected_) = inside_circle(pos, &circles) {
                *selected = Some(selected_);
            } else {
                *selected = None;
                draw_circle_lines(
                    absolute_pos.x,
                    absolute_pos.y,
                    RADIUS,
                    THICKNESS,
                    FAINT_CIRCLE_COLOR,
                );
            }
        };
    }
}

fn draw_circles(circles: &Vec<NormalizedPosition>, selected: Option<usize>) {
    let scale = 1.0;
    for (i, circle) in circles.iter().enumerate() {
        let absolute_pos = normalized_to_editor_absolute(*circle);
        if same(selected, i) {
            draw_circle(absolute_pos.x, absolute_pos.y, RADIUS, SETTLED_CIRCLE_COLOR);
        } else {
            draw_circle_lines(
                absolute_pos.x,
                absolute_pos.y,
                RADIUS,
                THICKNESS,
                SETTLED_CIRCLE_COLOR,
            );
        }

        let absolute_pos = normalized_to_canvas_absolute(*circle);

        for circle_1 in circles {
            let nested_pos = nest_pos(*circle, *circle_1, scale * 0.5);
            let absolute_pos_1 = normalized_to_canvas_absolute(nested_pos);
            draw_circle(
                absolute_pos_1.x,
                absolute_pos_1.y,
                RADIUS,
                SETTLED_CIRCLE_COLOR,
            );
        }
    }
}

fn same(maybe_selected: Option<usize>, index: usize) -> bool {
    if let Some(selected) = maybe_selected {
        selected == index
    } else {
        false
    }
}

fn inside_circle(pos: NormalizedPosition, circles: &Vec<NormalizedPosition>) -> Option<usize> {
    let normalized_radius = RADIUS / EDITOR_SIZE;
    let radius_squared = normalized_radius * normalized_radius;
    for (i, circle) in circles.iter().enumerate() {
        let diff = *circle - pos;
        let dot = diff.dot(diff);
        if dot < radius_squared {
            return Some(i);
        }
    }
    return None;
}
fn maybe_take(
    pos: NormalizedPosition,
    circles: &mut Vec<NormalizedPosition>,
) -> Option<NormalizedPosition> {
    return if let Some(found) = inside_circle(pos, circles) {
        let taken = circles.swap_remove(found);
        Some(taken)
    } else {
        None
    };
}

fn pos_in_editor((x, y): (f32, f32)) -> Option<NormalizedPosition> {
    return if x >= PAD && x < PAD + EDITOR_SIZE && y >= PAD && y < PAD + EDITOR_SIZE {
        Some(editor_absolute_to_normalized(Vec2::new(x, y)))
    } else {
        None
    };
}
fn nest_pos(pos: Vec2, nested_pos: Vec2, scale: f32) -> Vec2 {
    pos + nested_pos * scale
}

fn editor_absolute_to_normalized(pos: PixelPosition) -> NormalizedPosition {
    return Vec2::new((pos.x - PAD) / EDITOR_SIZE, (pos.y - PAD) / EDITOR_SIZE);
}
fn normalized_to_editor_absolute(pos: NormalizedPosition) -> PixelPosition {
    return Vec2::new(pos.x * EDITOR_SIZE + PAD, pos.y * EDITOR_SIZE + PAD);
}
fn normalized_to_canvas_absolute(pos: NormalizedPosition) -> PixelPosition {
    let canvas_x = 0.5 * (screen_width() - 4.0 * PAD - EDITOR_SIZE);
    let canvas_y = 0.5 * (screen_height() - 2.0 * PAD);
    return Vec2::new(
        pos.x * canvas_x + 3.0 * PAD + EDITOR_SIZE,
        pos.y * canvas_y + PAD,
    );
}
