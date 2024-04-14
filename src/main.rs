
mod error;
mod utils;
mod bodies;
mod config;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

fn main() -> Result<(), error::Error> {
    let sdl_ctx = sdl2::init()?;

    let video_ctx = sdl_ctx.video()?;

    let app_window = video_ctx.window("Kinetic sim",
        config::Window::WIDTH,
        config::Window::HEIGHT
        )
        .position(0, 0)
        .resizable()
        .opengl()
        .build()?;

    let mut canvas = app_window.into_canvas()
        .build()?;

    let canvas_box = bodies::rect::CanvasBox::default();

    // dev test 

    let mut sp = bodies::sphere::new();

        sp.add_velocity((0, 2));

    let mut event_pump = sdl_ctx.event_pump()?;

    'run: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } | Event::Quit {..} => break 'run,
                _ => println!("{:?}", event),
            }
        }
        // Process

        sp.update();


        // Render

        // BG
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        canvas_box.render(&mut canvas);


        sp.render(&mut canvas);

        canvas.present();



        //std::thread::sleep(std::time::Duration::from_millis(200));
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000_u32 / 60));
    }

    Ok(())
}
