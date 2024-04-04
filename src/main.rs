use std::time::Duration;
use ggez::{graphics, timer::sleep};

fn main() {
    let (ctx, _) = &mut ggez::ContextBuilder::new("hello_square", "ctrl-oddity")
        .window_setup(ggez::conf::WindowSetup::default().title("Hello Square!"))
        .window_mode(ggez::conf::WindowMode::default().resizable(true))
        .add_resource_path(&std::env::current_dir().unwrap())
        .build()
        .expect("Window creation failure!");

    let bounds = graphics::Rect::new(300.0, 190.0, 200.0, 200.0);
    let color = graphics::Color::from_rgb(255, 0, 0);
    let rect_mesh =
        graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), bounds, color).unwrap();

    graphics::clear(ctx, ggez::graphics::BLACK);
    graphics::draw(ctx, &rect_mesh, graphics::DrawParam::new()).unwrap();
    graphics::present(ctx).unwrap();

    sleep(Duration::new(1, 0));
}
