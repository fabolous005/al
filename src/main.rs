use draw::*;
use tokio::fs;
use warp::Filter;

#[tokio::main]
async fn main() {
    let mut canvas = Canvas::new(100, 100);

    // create a new drawing
    let mut rect = Drawing::new()
        // give it a shape
        .with_shape(Shape::Rectangle {
            width: 50,
            height: 50,
        })
        // move it around
        .with_xy(25.0, 25.0)
        // give it a cool style
        .with_style(Style::stroked(5, Color::black()));

    // add it to the canvas
    canvas.display_list.add(rect);

    // save the canvas as an svg
    render::save(&canvas, "test.svg", SvgRenderer::new()).expect("Failed to save");
}
