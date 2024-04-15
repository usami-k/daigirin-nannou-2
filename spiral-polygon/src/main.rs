use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    updated_time: f32,
    vertices: Vec<Point2>,
}

fn model(app: &App) -> Model {
    app.new_window().size(420, 420).view(view).build().unwrap();
    let num = 11;
    let vertices = (0..num)
        .map(|i| {
            let angle = (i as f32) * 2.0 * PI / (num as f32);
            let x = angle.cos() * 200.0;
            let y = angle.sin() * 200.0;
            pt2(x, y)
        })
        .collect();
    Model { updated_time: 0.0, vertices }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if app.time < model.updated_time + 0.5 {
        return;
    }
    model.updated_time = app.time;

    // それぞれの頂点を、隣の頂点に向かって少し移動する
    let vertices = &model.vertices;
    let num = vertices.len();
    model.vertices = (0..num)
        .map(|i| {
            let next = (i + 1) % num;
            vertices[i] + (vertices[next] - vertices[i]) * 0.23
        })
        .collect();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if app.elapsed_frames() == 0 {
        draw.background().color(WHITE);
    }
    draw.polyline()
        .points_closed(model.vertices.iter().cloned())
        .color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}
