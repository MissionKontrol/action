use nannou::prelude::*;

fn main() {
    nannou::app(model).simple_window(view).exit(my_exit).run()
}

fn view(app: &App, _model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    // Clear the background to blue.
    draw.background().color(DARKSEAGREEN);

    for (i,column) in _model.column_list.iter().enumerate() {
        draw.rect()
        .x_y(0.0 + (20.0 * i as f32), column.height/2.0)
        .w_h(column.width, column.height )
        .hsv(1.0, 1.0, 1.0);
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

struct Model {
    column_list: Vec<Column>,
}

fn model(_app: &App) -> Model {
    let mut column_list = Vec::new();
    let base_height = 20.0;

    for i in 0..5 {
        column_list.push(Column::new(base_height + (i as f32 * 10.0), 10.0));
    }
    Model {column_list}
}

fn my_exit(app: &App, model: Model){
    drop(model);
    drop(app); 
}
struct Column {
    height: f32,
    width: f32,
}

impl Column {
    fn new(height: f32, width: f32) -> Column {
        Column {
            height,
            width,
        }
    }
}