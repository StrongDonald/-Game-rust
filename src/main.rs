extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowContext};

use std::thread::sleep;
use std::time::{Duration, SystemTime};

use sdl2::image::{LoadTexture, InitFlag};

use std::fs::File;
use std::io::{self, Write, Read};

const TEXTURE_SIZE: u32 = 32;

// #[derive(Clone, Copy)]
// enum TextureColor {
//     Green,
//     Blue,
// }

fn main() {
    let sdl_context = sdl2::init()
    .expect("SDL initialization failed");
    
    
    let video_subsystem = sdl_context.video()
    .expect("Couldn't get SDL video subsystem");
    
    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
    .position_centered()
    .opengl()
    .build()
    .expect("Failed to create window");
    
    let mut canvas = window.into_canvas()
    .target_texture()
    .present_vsync()
    .build()
    .expect("Failed to convert window into canvas");
    
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    
    sdl2::image::init(InitFlag::PNG | InitFlag::JPG)
        .expect("Couldn't initialize image context");

    let image_texture = texture_creator.load_texture("assets/my_image.png")
        .expect("Couldn't load image");

    // let mut green_square = create_texture_rect(
    //     &mut canvas, 
    //     &texture_creator, 
    //     TextureColor::Green, 
    //     TEXTURE_SIZE
    // ).expect("Failed to create a texture.");


    // let mut blue_square = create_texture_rect(
    //     &mut canvas, 
    //     &texture_creator, 
    //     TextureColor::Blue, 
    //     TEXTURE_SIZE
    // ).expect("Failed to create a texture.");

    // let timer = SystemTime::now();

    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");

    'running: loop {
        canvas.set_draw_color(Color::RGB(125, 125, 125));
        canvas.clear();

        canvas.copy(
            &image_texture,
            None,
            None
        ).expect("Render failed");
        

        // let display_green = match timer.elapsed() {
        //     Ok(elapsed) => elapsed.as_secs() % 2 == 0,
        //     Err(_) => {
        //         true
        //     }
        // };

        // let square_texture = if display_green {
        //     &green_square
        // } else {
        //     &blue_square
        // };

        // canvas.copy(
        //     &square_texture,
        //     None,
        //     Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE)
        // ).expect("Drawing rectangle is Failed");

        canvas.present();
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
            sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

// fn create_texture_rect<'T>(
//     canvas: &mut Canvas<Window>,
//     texture_creator: &'T TextureCreator<WindowContext>,
//     color: TextureColor,
//     size: u32
// ) -> Option<Texture<'T>> {
//     if let Ok(mut square_texture) = 
//         texture_creator.create_texture_target(None, size, size) {
//             canvas.with_texture_canvas(&mut square_texture, |texture| {
//                 match color {
//                     TextureColor::Green => texture.set_draw_color(Color::RGB(0, 255, 0)),
//                     TextureColor::Blue => texture.set_draw_color(Color::RGB(0, 0, 255)),
//                 }
//                 texture.clear();
//             }).expect("Failed to color a texture");

//             Some(square_texture)
//         } else {
//             None
//         }
// }

fn write_into_file(content: &str, file_name: &str) -> io::Result<()> {
    let mut f = File::create((file_name))?;
    f.write_all(content.as_bytes())
}

fn read_from_file(file_name: &str) -> io::Result<String> {
    let mut f = File::open(file_name)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

fn slice_to_string(slice: &[u32]) -> String {
    slice.iter().map(|highscore| highscore.to_string())
        .collect::<Vec<String>>().join(" ")
}

fn save_highscores_and_lines(
    highscores: &[u32],
    number_of_lines: &[u32]
) -> bool {
    let s_highscores = slice_to_string(highscores);
    let s_number_of_lines = slice_to_string(number_of_lines);
    write_into_file(
        format!("{}\n{}\n", s_highscores, s_number_of_lines),
        "scores.txt"
    ).is_ok()
}

fn line_to_slice(line: &str) -> Vec<u32> {
    line.split(" ").filter_map(
        |nb| nb.parse::<u32>().ok()
    ).collect()
}

fn load_highscores_and_lines() -> Option<(Vec<u32>, Vec<u32>)> {
    if let Ok(content) = read_from_file("scores.txt") {
        let mut lines = content.splitn(2, "\n").map(
            |line| line_to_slice(line)
        ).collect::<Vec<_>>();

        if lines.len() == 2 {
            let (number_of_lines, highscores) = (
                lines.pop().unwrap(),
                lines.pop().unwrap()
            );

            Some((number_of_lines, highscores))
        } else {
            None
        }
    } else {
        None
    }
}