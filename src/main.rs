use macroquad::prelude::*;
use macroquad::ui::root_ui;

type Pixels = f32;
type NormalizedPosition = Vec2;
type PixelPosition = Vec2;
type AnyError = Box<dyn std::error::Error>;

const PAD: Pixels = 20.0;
const EDITOR_SIZE: Pixels = 200.0;
const FONT_SIZE: Pixels = 16.0;
const THICKNESS: Pixels = 2.0;
const RADIUS: Pixels = 10.0;
const MAX_DRAWN: i32 = 100000;

const FAINT_CIRCLE_COLOR: Color = Color::new(0.8, 0.8, 0.2, 0.2);
const STRONG_CIRCLE_COLOR: Color = Color::new(0.8, 0.8, 0.2, 0.7);
const SETTLED_CIRCLE_COLOR: Color = Color::new(0.8, 0.5, 0.2, 0.7);

struct State {
    circles: Vec<NormalizedPosition>,
    selected: Option<usize>,
    levels: i32,
}
impl State {
    pub fn new() -> Self {
        Self {
            circles: Vec::new(),
            selected: None,
            levels: 1,
        }
    }
}

#[macroquad::main("MY_CRATE_NAME")]
async fn main() {
    let mut state = State::new();
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(DARKGRAY);
        draw_rectangle_lines(PAD, PAD, EDITOR_SIZE, EDITOR_SIZE, THICKNESS, LIGHTGRAY);

        draw_buttons(&mut state);
        edit_circles(&mut state);
        let drawn = draw_circles(&state);
        draw_stats(&mut state, &drawn);
        next_frame().await
    }
}

fn draw_buttons(state: &mut State) {
    if root_ui().button(Vec2::new(PAD, screen_height() * 0.5), " + ") {
        state.levels += 1;
    }
    if state.levels > 0
        && root_ui().button(
        Vec2::new(PAD * 2.0 + FONT_SIZE, screen_height() * 0.5),
        " - ",
    )
    {
        state.levels -= 1;
    }
}

fn draw_stats(state: &mut State, drawn: &i32) {
    draw_text(
        &format!("points drawn: {}", drawn),
        PAD,
        screen_height() - PAD - FONT_SIZE,
        FONT_SIZE,
        BLACK,
    );
    draw_text(
        &format!("nesting levels: {}", state.levels),
        PAD,
        screen_height() - PAD - 2.0 * FONT_SIZE,
        FONT_SIZE,
        BLACK,
    );
    let score = compute_score(&state);
    draw_text(
        &format!("score: {}", score),
        PAD,
        screen_height() - PAD - 3.0 * FONT_SIZE,
        FONT_SIZE,
        BLACK,
    );

}

fn compute_score(state: &State) -> f32 {
    let mut score = 0.0;
    for (i, circle) in state.circles.iter().enumerate() {
        for (j, circle_2) in state.circles.iter().enumerate() {
            if j > i {
                let diff = *circle - *circle_2;
                score += diff.length()
            }
        }
    }
    score * state.levels as f32
}

fn edit_circles(
    State {
        circles, selected, ..
    }: &mut State,
) {
    let mouse_pos = mouse_position();
    if let Some(pos) = pos_in_editor(mouse_pos) {
        let mut absolute_pos = normalized_to_editor_absolute(pos);
        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some(selected_) = inside_circle(pos, &circles) {
                *selected = Some(selected_);
            } else {
                circles.push(pos);
                *selected = Some(circles.len() - 1);
            }
        }
        if is_mouse_button_down(MouseButton::Left) {
            if let Some(selected_) = selected {
                circles[*selected_] = pos;
            }
        } else {
            draw_circle_lines(
                absolute_pos.x,
                absolute_pos.y,
                RADIUS,
                THICKNESS,
                FAINT_CIRCLE_COLOR,
            );
        }
        if is_mouse_button_released(MouseButton::Left) {
            *selected = None;
        }
    } else {
        if is_mouse_button_released(MouseButton::Left) {
            if let Some(selected_) = selected {
                circles.swap_remove(*selected_);
                *selected = None;
            }
        }
    }
}

fn draw_circles(
    State {
        circles,
        selected,
        levels,
    }: &State,
) -> i32 {
    let mut drawn = 0;
    let mut too_many = false;
    for (i, circle) in circles.iter().enumerate() {
        let mut color = if same(*selected, i) {
            STRONG_CIRCLE_COLOR
        } else {
            SETTLED_CIRCLE_COLOR
        };
        if too_many {
            color = WHITE
        }
        let editor_pos = normalized_to_editor_absolute(*circle);
        if same(*selected, i) {
            draw_circle(editor_pos.x, editor_pos.y, RADIUS, color);
        } else {
            draw_circle_lines(editor_pos.x, editor_pos.y, RADIUS, THICKNESS, color);
        }

        if !too_many {
            let recursion = Recursion {
                level: *levels,
                scale: 1.0,
                radius: RADIUS,
            };
            if let Err(_) = draw_nested(recursion, circles, selected, *circle, color, &mut drawn) {
                too_many = true;
            }
        }
    }
    drawn
}

#[derive(Copy, Clone)]
struct Recursion {
    level: i32,
    scale: f32,
    radius: f32,
}
impl Recursion {
    fn reduce(&mut self) {
        self.level -= 1;
        self.scale *= 0.5;
        self.radius *= 0.75;
    }
    fn should_continue(&self) -> bool {
        self.level > 0
    }
    fn color(&self, color: Color) -> Color {
        let mut new_color = color;
        new_color.r = (color.r - self.level as f32 * 0.02).max(0.0);
        // new_color.b = (color.b + self.level as f32 * 0.05).max(0.0);
        new_color
    }
}

fn draw_nested(
    mut recursion: Recursion,
    circles: &Vec<NormalizedPosition>,
    selected: &Option<usize>,
    circle: NormalizedPosition,
    color: Color,
    drawn: &mut i32,
) -> Result<(), AnyError> {
    recursion.reduce();
    let absolute_pos = normalized_to_canvas_absolute(circle);
    let side = recursion.radius * 1.5;
    let drawing_color = recursion.color(color);
    if recursion.should_continue() {

        // draw_circle(absolute_pos.x, absolute_pos.y, recursion.radius, drawing_color);
        draw_rectangle(absolute_pos.x, absolute_pos.y, side, side, drawing_color);
    } else {
        draw_rectangle(absolute_pos.x, absolute_pos.y, side, side, drawing_color);
    }
    *drawn += 1;
    if *drawn > MAX_DRAWN {
        return Err("drawing more circles might freeze your computer".into());
    }
    if recursion.should_continue() {
        for (i_1, circle_1) in circles.iter().enumerate() {
            let color2 = if same(*selected, i_1) {
                STRONG_CIRCLE_COLOR
            } else {
                color
            };
            let nested_pos = nest_pos(circle, *circle_1, recursion.scale);
            draw_nested(recursion, circles, selected, nested_pos, color2, drawn)?;
        }
    }
    Ok(())
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
    pos + (nested_pos * scale).rotate(pos.normalize())
}

fn editor_absolute_to_normalized(pos: PixelPosition) -> NormalizedPosition {
    return Vec2::new((pos.x - PAD) / EDITOR_SIZE, (pos.y - PAD) / EDITOR_SIZE);
}
fn normalized_to_editor_absolute(pos: NormalizedPosition) -> PixelPosition {
    return Vec2::new(pos.x * EDITOR_SIZE + PAD, pos.y * EDITOR_SIZE + PAD);
}
fn normalized_to_canvas_absolute(pos: NormalizedPosition) -> PixelPosition {
    let sw = screen_width();
    let canvas_x = 0.3 * (screen_width() - 4.0 * PAD - EDITOR_SIZE);
    let canvas_y = canvas_x;
    return Vec2::new(pos.x * canvas_x + sw * 0.4, pos.y * canvas_y + sw * 0.15);
}
