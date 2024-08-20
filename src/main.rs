use macroquad::miniquad::date::now;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad::ui::widgets::Button;

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
const EDITOR: Rect = Rect {
    x: PAD,
    y: PAD + FONT_SIZE * 2.0,
    w: EDITOR_SIZE,
    h: EDITOR_SIZE,
};
const ARENA: Rect = Rect {
    x: 0.0,
    y: 0.0,
    w: 0.30,
    h: 0.30,
};

const FAINT_CIRCLE_COLOR: Color = Color::new(0.8, 0.8, 0.2, 0.2);
const STRONG_CIRCLE_COLOR: Color = Color::new(0.8, 0.8, 0.2, 0.7);
const SETTLED_CIRCLE_COLOR: Color = Color::new(0.8, 0.5, 0.2, 0.7);

struct State {
    circles: Vec<NormalizedPosition>,
    selected: Option<usize>,
    levels: i32,
    targets: Vec<PixelPosition>,
    accumulated_score: f32,
}
impl State {
    pub fn new() -> Self {
        Self {
            circles: Vec::new(),
            selected: None,
            levels: 1,
            targets: Vec::new(),
            accumulated_score: 0.0,
        }
    }
}

#[macroquad::main("MY_CRATE_NAME")]
async fn main() {
    let mut state = State::new();
    let mut arena = calculate_arena(ARENA);
    let random_target = create_random_target(now(), arena);
    state.targets.push(random_target);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        arena = calculate_arena(ARENA); // redo in case of window resize
        clear_background(DARKGRAY);

        draw_editor();
        let after_instructions_pos = draw_instructions();

        draw_buttons(&mut state);
        edit_circles(&mut state);
        let (drawn, touching_targets) = draw_circles(&state);
        maybe_create_target(&mut state, arena, after_instructions_pos, touching_targets);
        draw_stats(&mut state, &drawn);
        draw_arena(arena, &state);
        next_frame().await
    }
}

fn maybe_create_target(
    state: &mut State,
    arena: Rect,
    after_instructions_pos: PixelPosition,
    touching_targets: bool,
) {
    if !touching_targets {
        if root_ui().button(after_instructions_pos, " Next target ") {
            let score = compute_score(&state);
            state.accumulated_score += score;
            let target = create_random_target(now(), arena);
            state.targets.push(target);
            // println!("targets:");
            // for target in &state.targets {
            //     println!("{}", target);
            // }
        }
    }
    if root_ui().button(
        after_instructions_pos + Vec2::new(FONT_SIZE * 8.0, 0.0),
        " Restart ",
    ) {
        state.targets.clear();
    }
}

fn create_random_target(ts: f64, arena: Rect) -> PixelPosition {
    normalized_to_rect(create_random_target_normalized(ts), arena)
}
fn create_random_target_normalized(ts: f64) -> NormalizedPosition {
    Vec2::new(
        (ts.fract() * 37.0).fract().clamp(0.0, 1.0) as f32,
        (ts.fract() * (57.0 + ts.round() % 7.0))
            .fract()
            .clamp(0.0, 1.0) as f32,
    )
}

fn draw_editor() {
    draw_text(&"Editor:", PAD, PAD + FONT_SIZE, FONT_SIZE, LIGHTGRAY);
    draw_rectangle_lines(EDITOR.x, EDITOR.y, EDITOR.w, EDITOR.h, THICKNESS, LIGHTGRAY);
}
fn draw_instructions() -> PixelPosition {
    let x = PAD * 4.0 + EDITOR_SIZE;
    let mut y = PAD + FONT_SIZE * 1.0;
    draw_text(
        &"Click in the editor on the left to add points.",
        x,
        y,
        FONT_SIZE,
        LIGHTGRAY,
    );
    y += FONT_SIZE;
    draw_text(
        &"Increase the nesting levels to create more points.",
        x,
        y,
        FONT_SIZE,
        LIGHTGRAY,
    );
    y += FONT_SIZE;
    draw_text(
        &"Move your points to avoid touching the blue targets.",
        x,
        y,
        FONT_SIZE,
        LIGHTGRAY,
    );
    y += FONT_SIZE;
    return Vec2::new(x, y);
}

fn calculate_arena(normalized: Rect) -> Rect {
    let Vec2 { x, y } = normalized_to_canvas_absolute(normalized.point());
    let Vec2 { x: w, y: h } = normalized_to_canvas_absolute(normalized.size());
    Rect::new(x * 0.6, y, w, h)
}
fn draw_arena(arena: Rect, state: &State) {
    draw_rectangle_lines(arena.x, arena.y, arena.w, arena.h, 2.0, BLACK);
    for target in &state.targets {
        let absolute_pos = *target;
        draw_circle(absolute_pos.x, absolute_pos.y, RADIUS, BLUE);
    }
}

fn draw_buttons(state: &mut State) {
    let text_width = FONT_SIZE * 8.5;
    if root_ui().button(
        Vec2::new(PAD + text_width, screen_height() * 0.5 - FONT_SIZE),
        " + ",
    ) {
        state.levels += 1;
    }
    if state.levels > 0
        && root_ui().button(
            Vec2::new(
                PAD * 2.0 + FONT_SIZE + text_width,
                screen_height() * 0.5 - FONT_SIZE,
            ),
            " - ",
        )
    {
        state.levels -= 1;
    }
}

fn draw_stats(state: &mut State, drawn: &i32) {
    let line_height = 1.5 * FONT_SIZE;
    draw_text(
        &format!("nesting levels: {}", state.levels),
        PAD,
        screen_height() * 0.5,
        FONT_SIZE,
        LIGHTGRAY,
    );
    draw_text(
        &format!("points drawn: {}", drawn),
        PAD,
        screen_height() * 0.5 + line_height,
        FONT_SIZE,
        LIGHTGRAY,
    );
    let score = compute_score(&state);
    draw_text(
        &format!("score: {}", score),
        PAD,
        screen_height() * 0.5 + 2.0 * line_height,
        FONT_SIZE,
        LIGHTGRAY,
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
        ..
    }: &State,
) -> (i32, bool) {
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
            if let Err(_) = draw_nested(
                recursion,
                circles,
                selected,
                Vec2::default(),
                *circle,
                color,
                &mut drawn,
            ) {
                too_many = true;
            }
        }
    }
    (drawn, false)
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
    reference: NormalizedPosition,
    circle: NormalizedPosition,
    color: Color,
    drawn: &mut i32,
) -> Result<(), AnyError> {
    recursion.reduce();
    let absolute_pos = normalized_to_canvas_absolute(circle);
    let absolute_pos_ref = normalized_to_canvas_absolute(reference);
    let side = recursion.radius * 1.5;
    let drawing_color = recursion.color(color);
    draw_line(
        absolute_pos_ref.x,
        absolute_pos_ref.y,
        absolute_pos.x,
        absolute_pos.y,
        1.0,
        color,
    );
    if recursion.should_continue() {

        // draw_circle(absolute_pos.x, absolute_pos.y, recursion.radius, drawing_color);
        // draw_rectangle(absolute_pos.x, absolute_pos.y, side, side, drawing_color);
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
            draw_nested(
                recursion, circles, selected, circle, nested_pos, color2, drawn,
            )?;
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
    let point = Vec2::new(x, y);
    return if EDITOR.contains(point) {
        Some(editor_absolute_to_normalized(point))
    } else {
        None
    };
}
fn nest_pos(pos: Vec2, nested_pos: Vec2, scale: f32) -> Vec2 {
    pos + (nested_pos * scale).rotate(pos.normalize())
}

fn editor_absolute_to_normalized(pos: PixelPosition) -> NormalizedPosition {
    rect_to_normalized(pos, EDITOR)
}
fn normalized_to_editor_absolute(pos: NormalizedPosition) -> PixelPosition {
    normalized_to_rect(pos, EDITOR)
}
fn normalized_to_rect(pos: NormalizedPosition, rect: Rect) -> PixelPosition {
    Vec2::new(pos.x * rect.w + rect.x, pos.y * rect.h + rect.y)
}
fn rect_to_normalized(pos: NormalizedPosition, rect: Rect) -> PixelPosition {
    Vec2::new((pos.x - rect.x) / rect.w, (pos.y - rect.y) / rect.h)
}
fn normalized_to_canvas_absolute(pos: NormalizedPosition) -> PixelPosition {
    let sw = screen_width();
    let canvas_x = 0.3 * (screen_width() - 4.0 * PAD - EDITOR_SIZE);
    let canvas_y = canvas_x;
    return Vec2::new(pos.x * canvas_x + sw * 0.57, pos.y * canvas_y + sw * 0.28);
}
