use sdl2::{event::Event, keyboard::Keycode, render::WindowCanvas, video::GLProfile, Sdl};
use std::f32;

struct Game {
    sdl_context: Sdl,
    canvas: WindowCanvas,
    r: f32,
    g: f32,
    b: f32,
}
fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn build_window() -> Game {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("Window", 800, 600)
        .opengl()
        .build()
        .unwrap();

    // Unlike the other example above, nobody created a context for your window, so you need to create one.
    let canvas = window
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();
    let _ = canvas.window().gl_set_context_to_current();

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    return Game {
        sdl_context,
        canvas,
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
}
fn main() {
    let mut game = build_window();
    let mut event_pump = game.sdl_context.event_pump().unwrap();

    'running: loop {
        unsafe {
            gl::ClearColor(game.r, game.g, game.b, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    if game.r >= 1.0 {
                        game.r = 0.0;
                    }
                    game.r = game.r + 0.1
                }
                Event::KeyDown {
                    keycode: Some(Keycode::G),
                    ..
                } => {
                    if game.g >= 1.0 {
                        game.g = 0.0;
                    }
                    game.g = game.g + 0.1
                }
                Event::KeyDown {
                    keycode: Some(Keycode::B),
                    ..
                } => {
                    if game.b > 1.0 {
                        game.b = 0.0;
                    }
                    game.b = game.b + 0.1
                }

                _ => {}
            }
        }
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
        game.canvas.present();
    }
}
