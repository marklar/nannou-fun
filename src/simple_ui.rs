use nannou::prelude::*;
use nannou::ui::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    ui: Ui,
    ids: Ids,
    num_sides: usize,
    scale: f32,
    rotation: f32,
    color: Rgb,
    position: Point2,
}

widget_ids! {
    struct Ids {
        num_sides,
        scale,
        rotation,
        random_color,
        position,
    }
}

fn model(app: &App) -> Model {
    // Set the loop mode to wait for events, an energy-efficient option for pure-GUI apps.
    app.set_loop_mode(LoopMode::Wait);

    // Create the UI.
    let mut ui = app.new_ui().build().unwrap();

    // Generate some ids for our widgets.
    let ids = Ids::new(ui.widget_id_generator());

    // Init our variables
    let num_sides = 5;
    let scale = 200.0;
    let rotation = 0.0;
    let position = pt2(0.0, 0.0);
    let color = rgb(1.0, 0.0, 1.0);

    Model {
        ui,
        ids,
        num_sides,
        scale,
        rotation,
        position,
        color,
    }
}


const SLIDER_WIDTH: f64 = 200.0;
const SLIDER_HEIGHT: f64 = 30.0;

const PAD_WIDTH: f64 = 200.0;
const PAD_HEIGHT: f64 = 200.0;

const SPACE_MAX_X: f32 = 300.0;
const SPACE_MAX_Y: f32 = 300.0;


fn update(_app: &App, model: &mut Model, _update: Update) {
    // Calling `set_widgets` allows us to instantiate some widgets.
    let ui_cell: &mut UiCell = &mut model.ui.set_widgets();

    // Create a slider.
    fn build_slider(label: &'static str, val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
        widget::Slider::new(val, min, max)
            .skew(1.0)  // default -- closer to 0, more values at low end of slider
            .enabled(false)  // default -- if false, merely displays value, doesn't allow modification?
            // Sizeable
            .w_h(SLIDER_WIDTH, SLIDER_HEIGHT)
            // Colorable (grey)
            .rgb(0.3, 0.3, 0.3)
            // Borderable (none)
            .border(0.0)
            // Labelable
            .label(label)              // text
            .label_font_size(15)       // font
            .label_rgb(1.0, 1.0, 1.0)  // color (white)
    }

    // (With each update, each slider is created anew.
    // It's not obvious to me why this is part of the Model.
    // Why not just part of the view?)

    let sides_slider = build_slider("N-gon", model.num_sides as f32, 3.0, 15.0)
        .top_left_with_margin(20.0)     // Positionable: in top-left corner of parent, w/ margin
        .set(model.ids.num_sides, ui_cell);  // Adds the slider to 'ui'.
    // Update the model from the slider.
    // The slider is an Iterator? Exhaust it; use the last value to set our model?
    for value in sides_slider { model.num_sides = value as usize; }

    let scale_slider = build_slider("Scale", model.scale, 10.0, 500.0)
        .down(10.0)    // Positionable: down 10px from previous widget
        .set(model.ids.scale, ui_cell);
    for value in scale_slider { model.scale = value; }

    let rot_slider = build_slider("Rotation", model.rotation, -PI, PI)
        .down(10.0)   // Positionable: down 10px from previous widget
        .set(model.ids.rotation, ui_cell);
    for value in rot_slider { model.rotation = value; }

    let color_button = widget::Button::new()
        // Positionable trait: down 10px from previous widget
        .down(10.0)
        // Sizeable trait
        .w_h(100.0, 60.0)
        // Colorable trait
        .rgb(0.3, 0.3, 0.3)
        // Borderable trait
        .border(1.0)  // width
        .border_rgba(10.0, 10.0, 10.0, 1.0)  // color & opacity
        // Labelable trait
        .label("Random\nColor")
        .label_font_size(15)
        .label_rgb(1.0, 1.0, 1.0)
        // After building the widget...
        // Set its current state & style into the given Ui.
        .set(model.ids.random_color, ui_cell);
    for _click in color_button {
        // (We don't use any info from the click.)
        model.color = rgb(random(), random(), random());
    }

    // Mini 'touch pad' for moving the shape.
    // (Why not just manipulate the shape directly?)
    let xy_pad: widget::XYPad<f32, f32> =
        widget::XYPad::new(
            model.position.x, -SPACE_MAX_X, SPACE_MAX_X,
            model.position.y, -SPACE_MAX_Y, SPACE_MAX_Y,
        )
        .down(10.0)
        .w_h(PAD_WIDTH, PAD_HEIGHT)  // only 1/2 as large as the area it represents
        .label("Position")
        .label_font_size(15)
        .label_rgb(1.0, 1.0, 1.0)
        .rgb(0.3, 0.3, 0.3)
        .border(0.0);
    for (x, y) in xy_pad.set(model.ids.position, ui_cell) {
        model.position = Point2::new(x, y);
    }
}

// fn add_xy_pad(model: &mut Model) {
// }

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    draw.background().rgb(0.02, 0.02, 0.02);

    draw.ellipse()
        .xy(model.position)
        .radius(model.scale)
        .resolution(model.num_sides)
        .rotate(model.rotation)
        .color(model.color);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // Draw the state of the `Ui` to the frame.
    model.ui.draw_to_frame(app, &frame).unwrap();
}
