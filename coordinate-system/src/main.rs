use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window().size(420, 420).view(view).build().unwrap();
    Model {}
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    let win = app.window_rect();
    draw.arrow()
        .points(win.mid_left(), win.mid_right())
        .head_length(12.0)
        .head_width(6.0)
        .color(GRAY);
    draw.text("X").xy(win.mid_right() + pt2(-16.0, -16.0)).color(GRAY);
    draw.arrow()
        .points(win.mid_bottom(), win.mid_top())
        .head_length(12.0)
        .head_width(6.0)
        .color(GRAY);
    draw.text("Y").xy(win.mid_top() + pt2(-16.0, -16.0)).color(GRAY);
    draw.text("(0,0)").xy(pt2(0.0, 0.0)).color(BLACK);
    draw.polyline()
        .points([pt2(120.0, 0.0), pt2(120.0, 70.0), pt2(0.0, 70.0)])
        .color(LIGHTGRAY);
    draw.text("(120,0)").xy(pt2(120.0, 0.0)).color(BLACK);
    draw.text("(120,70)").xy(pt2(120.0, 70.0)).color(BLACK);
    draw.text("(0,70)").xy(pt2(0.0, 70.0)).color(BLACK);
    draw.polyline()
        .points([pt2(-60.0, 0.0), pt2(-60.0, -90.0), pt2(0.0, -90.0)])
        .color(LIGHTGRAY);
    draw.text("(-60,-90)").xy(pt2(-60.0, -90.0)).color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}
