use nannou::prelude::*;
use nannou::ui::prelude::*;


fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

widget_ids! {
    struct Ids {
        speed,
        position,
        spread,
        scale,
        theta,
        size,
        add_ball,
        rate,
    }
}

struct Bulb {
    color: rgb::Srgb<u8>,
    t: f32,
}

struct Model {
    ui: Ui,
    ids: Ids,
    bulbs: Vec<Bulb>,
    rate : f32,
    speed : f32,
    position : Point2,
    spread : f32,
    scale: f32,
    theta: f32,
    size : f32,
}

fn model(_app: &App) -> Model {
    let mut ui = _app.new_ui().build().unwrap();
        
    // Generate some ids for our widgets.
    let ids = Ids::new(ui.widget_id_generator());
    
    let speed = 1.;
    let center_y = 500.0;
    let center_x = 0.0;
    let spread = 2.0;
    let meter_to_pixel = 1100.;
    let theta = PI / 3.3;
    let size = 40.0;
    let rate = 31.0;
     
    // [
    //         Bulb { color: RED, t: 31.0 / 60.0,},
    //         Bulb { color: BLUE, t: 32.0 / 60.0},
    //         Bulb { color: GREEN, t: 33.0 / 60.0},
    //         Bulb { color: YELLOW, t: 34.0 / 60.0},
    //         Bulb { color: ORANGE, t: 35.0 / 60.0},
    //         Bulb { color: PURPLE, t: 36.0 / 60.0},
    //         Bulb { color: CYAN, t: 37.0 / 60.0},
    //         Bulb { color: WHITE, t: 38.0 / 60.0},
    //         Bulb { color: GREY, t: 39.0 / 60.0},
    //         Bulb { color: HONEYDEW, t: 40.0 / 60.0},
    //         Bulb { color: LIME, t: 41.0 / 60.0},
    //         Bulb { color: ORCHID, t: 42.0 / 60.0},
    //         Bulb { color: POWDERBLUE, t: 43.0 / 60.0},
    //         Bulb { color: LIGHTCORAL, t: 44.0 / 60.0},
    //         Bulb { color: LIGHTBLUE, t: 45.0 / 60.0},
    //         Bulb { color: KHAKI, t: 46.0 / 60.0},
    //         Bulb { color: IVORY, t: 47.0 / 60.0},
    //         Bulb { color: GAINSBORO, t: 48.0 / 60.0},
    //         Bulb { color: RED, t: 49.0 / 60.0,},
    //         Bulb { color: BLUE, t: 50.0 / 60.0},
    //         Bulb { color: GREEN, t: 51.0 / 60.0},
    //         Bulb { color: YELLOW, t: 52.0 / 60.0},
    //         Bulb { color: ORANGE, t: 53.0 / 60.0},
    //         Bulb { color: PURPLE, t: 54.0 / 60.0},
    //         Bulb { color: CYAN, t: 55.0 / 60.0},
    //         Bulb { color: WHITE, t: 56.0 / 60.0},
    //         Bulb { color: GREY, t: 57.0 / 60.0},
    //         Bulb { color: HONEYDEW, t: 58.0 / 60.0},
    //         Bulb { color: LIME, t: 59.0 / 60.0},
    //         Bulb { color: ORCHID, t: 60.0 / 60.0},
    //         Bulb { color: POWDERBLUE, t: 61.0 / 60.0},
    //         Bulb { color: LIGHTCORAL, t: 62.0 / 60.0},
    //         Bulb { color: LIGHTBLUE, t: 63.0 / 60.0},
    //         Bulb { color: KHAKI, t: 64.0 / 60.0},
    //         Bulb { color: IVORY, t: 65.0 / 60.0},
    //         Bulb { color: GAINSBORO, t: 66.0 / 60.0},
    //         ]

    let m = Model { 
        ui : ui,
        ids : ids,
        rate : rate,
        bulbs: vec![Bulb { color: GAINSBORO, t: 0.0}],
        speed : speed,
        position : Point2 {x: center_x, y: center_y},
        spread : spread,
        scale : meter_to_pixel,
        theta : theta,
        size : size,
        };

    return m;
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // Calling `set_widgets` allows us to instantiate some widgets.
    let ui = &mut _model.ui.set_widgets();

    fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
        widget::Slider::new(val, min, max)
            .w_h(200.0, 30.0)
            .label_font_size(15)
            .rgb(0.3, 0.3, 0.3)
            .label_rgb(1.0, 1.0, 1.0)
            .border(0.0)
    }

    for value in slider(_model.speed as f32, 0.1, 10.0)
        .top_left_with_margin(20.0)
        .label("Speed")
        .set(_model.ids.speed, ui)
    {
        _model.speed = value as f32;
    }

    for value in slider(_model.spread as f32, 0.1, 10.0)
        .down(10.0)
        .label("Spread")
        .set(_model.ids.spread, ui)
    {
        _model.spread = value as f32;
    }

    for value in slider(_model.scale as f32, 100.0, 2000.0)
        .down(10.0)
        .label("Scale")
        .set(_model.ids.scale, ui)
    {
        _model.scale = value as f32;
    }

    for value in slider(_model.theta, -PI, PI)
        .down(10.0)
        .label("Theta")
        .set(_model.ids.theta, ui)
    {
        _model.theta = value;
    }

    for value in slider(_model.size as f32, 5.0, 100.0)
        .down(10.0)
        .label("Size")
        .set(_model.ids.size, ui)
    {
        _model.size = value as f32;
    }

    for value in slider(_model.rate as f32, 1.0, 360.0)
        .down(10.0)
        .label("Rate")
        .set(_model.ids.rate, ui)
    {
        _model.rate = value as f32;
    }

    for _click in widget::Button::new()
        .down(10.0)
        .w_h(200.0, 60.0)
        .label("Add Ball")
        .label_font_size(15)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .set(_model.ids.add_ball, ui)
    {
        _model.bulbs.push(Bulb {color: rgb(random(), random(), random()), t: 0.0});
    }

    for (x, y) in widget::XYPad::new(
        _model.position.x,
        -1000.0,
        1000.0,
        _model.position.y,
        -1000.0,
        1000.0,
    )
    .down(10.0)
    .w_h(200.0, 200.0)
    .label("Position")
    .label_font_size(15)
    .rgb(0.3, 0.3, 0.3)
    .label_rgb(1.0, 1.0, 1.0)
    .border(0.0)
    .set(_model.ids.position, ui)
    {
        _model.position = Point2::new(x, y);
    }
}

fn view(_app: &App, _model: &Model, frame: Frame){
    let draw = _app.draw();
    let mut i = 0.0;

    draw.background().color(BLACK); 
    for bulb in _model.bulbs.iter() {
        let t = (_model.rate + i) as f32 / 60.0;
        let sine =  (_model.theta * (_model.speed * _app.time * t).cos() - (PI / 2.)).sin();
        let cosine =  (_model.theta * (_model.speed * _app.time * t).cos() - (PI / 2.)).cos();
        let radius = ((t * _model.spread) / (2. * PI)).pow(2.) * 9.81 * _model.scale;
        draw.ellipse().w(_model.size).h(_model.size).color(bulb.color).x_y((cosine * radius) + _model.position.x, (sine * radius) + _model.position.y);
        i += 1.0;
    }
    
    draw.to_frame(_app, &frame).unwrap();
    // println!("time: {}", t)

    // Draw the state of the `Ui` to the frame.
    _model.ui.draw_to_frame(_app, &frame).unwrap();
}