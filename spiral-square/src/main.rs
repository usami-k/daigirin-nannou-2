use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    updated_time: f32,
    vertex1: Point2,
    vertex2: Point2,
    vertex3: Point2,
    vertex4: Point2,
}

fn model(app: &App) -> Model {
    app.new_window().size(420, 420).view(view).build().unwrap();
    Model {
        updated_time: 0.0,
        // 最初の正方形の頂点座標
        vertex1: pt2(200.0, 200.0),
        vertex2: pt2(-200.0, 200.0),
        vertex3: pt2(-200.0, -200.0),
        vertex4: pt2(200.0, -200.0),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if app.time < model.updated_time + 0.5 {
        return;
    }
    model.updated_time = app.time;

    // それぞれの頂点を、隣の頂点に向かって少し移動する
    let vertex1 = model.vertex1;
    let vertex2 = model.vertex2;
    let vertex3 = model.vertex3;
    let vertex4 = model.vertex4;
    model.vertex1 = vertex1 + (vertex2 - vertex1) * 0.1;
    model.vertex2 = vertex2 + (vertex3 - vertex2) * 0.1;
    model.vertex3 = vertex3 + (vertex4 - vertex3) * 0.1;
    model.vertex4 = vertex4 + (vertex1 - vertex4) * 0.1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if app.elapsed_frames() == 0 {
        draw.background().color(WHITE);
    }
    draw.polyline()
        .points_closed(vec![
            model.vertex1,
            model.vertex2,
            model.vertex3,
            model.vertex4,
        ])
        .color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}
