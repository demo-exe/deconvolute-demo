extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys
        .window("Rust SDL test!", SCREEN_WIDTH, SCREEN_HEIGHT)
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture(
            Some(pixels::PixelFormatEnum::RGBA8888),
            sdl2::render::TextureAccess::Streaming,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
        .unwrap();

    let mut data: [u8; (SCREEN_HEIGHT * SCREEN_WIDTH * 4) as usize] =
        [0; (SCREEN_HEIGHT * SCREEN_WIDTH * 4) as usize];

    let mut events = sdl_context.event_pump()?;

    'main: loop {
        for event in events.wait_timeout_iter(1) {
            match event {
                Event::Quit { .. } => break 'main,

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Escape {
                        break 'main;
                    } else if keycode == Keycode::Space {
                        println!("space down");
                        texture
                            .update(None, &data, (SCREEN_WIDTH * 4) as usize)
                            .unwrap();

                        let mut i: u8 = 0;
                        for n in 0..(SCREEN_HEIGHT * SCREEN_WIDTH * 4) {
                            // set data to random number
                            data[n as usize] = i;
                            i += 1;
                            if i == 255 {
                                i = 0;
                            }
                        }
                        canvas.copy(&texture, None, None).unwrap();
                        canvas.present();
                    }
                }

                _ => {}
            }
        }
    }

    Ok(())
}
