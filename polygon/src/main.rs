use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {
    vertices: Vec<Point2>,
}

fn model(app: &App) -> Model {
    app.new_window().size(420, 420).view(view).build().unwrap();
    // 正多角形の頂点数
    let num = 11;
    // 正多角形の頂点座標
    let vertices = (0..num)
        .map(|i| {
            // i番目の頂点の偏角
            let angle = (i as f32) * 2.0 * PI / (num as f32);
            // x座標とy座標を計算する
            let x = angle.cos() * 200.0;
            let y = angle.sin() * 200.0;
            pt2(x, y)
        })
        .collect();
    Model { vertices }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    draw.polyline()
        .points_closed(model.vertices.iter().cloned())
        .color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}
