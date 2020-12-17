// 'prelude': convenient access to most-common items
use nannou::prelude::*;
use nannou::noise::*;

fn main() {
    // build the app
    nannou::app(init_model) // Specify the initial model.
        .event(event)       // Handle all app events.
        .update(update)     // Handle timed updates, specifically.
        .run();             // Run the app.
}

//------------------------
// MODEL

type Color = Rgb<u8>;

struct Thing {
    position: Vector2,
    color: Color,
    radius: f32
}
impl Thing {
    pub fn new(position: Vector2, color: Color, radius: f32) -> Self {
        Thing { position, color, radius }
    }
}

// app state. representation of our program at any moment.
//
// We can update the model as certain events occur --
// such as mouse presses, key presses, or timed updates.
struct Model {
    // Why the '_'?
    // _window: window::Id,
    things: Vec<Thing>,
    noise: Perlin
}

const N_THINGS: usize = 2000;
const SCREEN_WIDTH: u32 = 2048;
const SCREEN_HEIGHT: u32 = 1400;


// Return a number between (-0.5 * sz) and (0.5 * sz).
// The origin (0,0) is in the MIDDLE.
fn random_pt() -> Vector2<f32> {
    fn random_offset(sz: f32) -> f32 {
        (random::<f32>() - 0.5) * sz
    }
    let x = random_offset(SCREEN_WIDTH as f32);
    let y = random_offset(SCREEN_HEIGHT as f32);
    vec2(x, y)
}

fn random_color() -> Color {
    // TODO: More officient way to generate random numbers.
    // (This is called from a tight loop.)
    let colors = [DODGERBLUE, WHITE, STEELBLUE, LIGHTSKYBLUE, DARKTURQUOISE];
    let idx = random::<usize>() % colors.len();
    colors[idx]
}


// Run just once, at the beginning.
//
// The "setup" stage of our app. Do things such as:
//  - create some windows,
//  - create a GUI,
//  - load some images,
//  - load data files, etc.
//
// Providing 'app' helps us create I/Os
// (windows, GUIs, audio streams, etc.).
//
// The 'App' type wraps up the finicky details of the app
// (e.g. establishing event loops, spawning I/O streams, etc.)
// and provides a nice, high-level API.
fn init_model(app: &App) -> Model {
    // From the 'app', create a window.
    let _window = app
        .new_window().size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .view(view)   // It's rendered via the 'view' fn.
        .build().unwrap();

    let noise = Perlin::new();
    Model { things: mk_things(), noise }
}


fn mk_things() -> Vec<Thing> {
    // Is there a nice FP way to build this Vec?
    let mut things = Vec::new();
    for _i in 0..N_THINGS {
        let radius = 1.5 + (random::<f32>() * 2.0);
        let t = Thing::new(random_pt(), random_color(), radius);
        things.push(t)
    }
    things
}

//------------------------
// UPDATE fns


// Run every time some kind of app event occurs.
//   - mouse & keyboard presses,
//   - window resizes,
//   - timed updates, etc.
// Reasons to update our Model.
//
// We can either:
//   - use this event, pattern matching on Event ctors, OR
//   - register more-specific functions for only those events
//     we care about (e.g. 'update' for timed updates [frames])
fn event(
    _app: &App,
    _model: &mut Model,
    _event: Event
) {
}


// For each animation frame (i.e. Update event).
fn update(
    _app: &App,
    model: &mut Model,   // The purpose of 'update': mutate Model.
    _update: Update      // A timed update event.
) {
    // 'Adding' two vectors gets you to a new point.
    for thing in model.things.iter_mut() {
        let change = jiggle();
        // let change = noise_jiggle(model.noise, thing.position);
        thing.position += change;
    }
}


fn noise_jiggle(noise: Perlin, pt: Vector2<f32>) -> Vector2<f32> {
    // What is the noise at the position of my thing?
    // Noise is not meant to work over the range of the entire screen.
    // It's meant to work on very small scales.
    let sn = 0.01;  // 0.01    // scaling factor
    let noise_0 = noise.get([
        sn * pt.x as f64,
        sn * pt.y as f64,
        0.0  // 3rd dimension?
    ]);
    let noise_1 = noise.get([
        sn * pt[0] as f64,
        sn * pt[1] as f64,
        1.0  // 3rd dimension?
    ]);
    vec2(noise_0 as f32, noise_1 as f32)
}

// Return a _small_ 2d vector.
fn jiggle() -> Vector2<f32> {
    fn bump() -> f32 { 3.0 * (random::<f32>() - 0.5) }
    vec2(bump(), bump())
}


fn mouse_moved(
    _app: &App,
    _model: &mut Model,
    _pos: nannou::prelude::Point2         // Where the mouse moved _to_?
) {
}

fn mouse_wheel(
    _app: &App,
    _model: &mut Model,
    _dt: MouseScrollDelta,
    _phase: TouchPhase
) {
}

//------------------------

fn view(
    app: &App,
    model: &Model,   // We can only _use_ the Model. (Mutating happens before.)
    frame: Frame
) {
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        // Start out w/ black background.
        draw.background().color(BLACK);
    } else {
        // _Almost_ black out the screen.
        let mostly_transparent_black = srgba(0.0, 0.0, 0.0, 0.05);
        draw.rect()
            .w_h(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)
            .color(mostly_transparent_black);
    }

    // 'elapsed' = frames since start of program.
    // Divide by 60 => ~seconds (~60 fps).
    // let time = app.elapsed_frames() as f32 / 60.0;

    for t in model.things.iter() {
        draw.ellipse()
            .radius(t.radius)
            .color(t.color)
            .x_y(t.position[0], t.position[1]);
    }
    
    draw.to_frame(app, &frame).unwrap();
}

fn view_revolving_circles(
    app: &App,
    _model: &Model,   // We can only _use_ the Model. (Mutating happens before.)
    frame: Frame
) {
    let draw = app.draw();
    draw.background().color(BLACK);

    // 'elapsed' = frames since start of program.
    // Divide by 60 => ~seconds (~60 fps).
    let time = app.elapsed_frames() as f32 / 60.0;
    let num_circles = 8;
    for i in 0..num_circles {
        let color: Color = if i % 2 == 0 { STEELBLUE } else { WHITE };
        let angle = time + (i as f32 / num_circles as f32 * TAU);
        draw.ellipse()
            .x_y(
                100.0 * angle.cos(),
                100.0 * angle.sin()
            )
            .color(color);
    }

    draw.to_frame(app, &frame).unwrap();
}
