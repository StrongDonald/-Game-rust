extern crate sdl2;
extern crate rand;

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

struct Tetrimino {
    states: Vec<Vec<Vec<u8>>>,
    x: isize,
    y: usize,
    current_state: u8,
}

trait TetriminoGenerator {
    fn new() -> Tetrimino;
}

struct TetriminoI;

impl TetriminoGenerator for TetriminoI {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![1, 1, 1, 1],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                ]
            ],
            x: 4,
            y: 0,
            current_state: 0
        }
    }
}

struct TetriminoJ;

impl TetriminoGenerator for TetriminoJ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![2, 2, 2, 0],
                    vec![2, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![2, 2, 0, 0],
                    vec![0, 2, 0, 0],
                    vec![0, 2, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 0, 2, 0],
                    vec![2, 2, 2, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![2, 0, 0, 0],
                    vec![2, 0, 0, 0],
                    vec![2, 2, 0, 0],
                    vec![0, 0, 0, 0],
                ]
            ],
            x: 4,
            y: 0,
            current_state: 0
        }
    }
}

struct TetriminoL;

impl TetriminoGenerator for TetriminoL {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![3, 3, 3, 0],
                    vec![0, 0, 3, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 3, 0, 0],
                    vec![0, 3, 0, 0],
                    vec![3, 3, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![3, 0, 0, 0],
                    vec![3, 3, 3, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![3, 3, 0, 0],
                    vec![3, 0, 0, 0],
                    vec![3, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ]
            ],
            x: 4,
            y: 0,
            current_state: 0
        }
    }
}

struct TetriminoO;

impl TetriminoGenerator for TetriminoO {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![4, 4, 0, 0],
                    vec![4, 4, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ]
            ],
            x: 5,
            y: 0,
            current_state: 0
        }
    }
}

struct TetriminoS;

impl TetriminoGenerator for TetriminoS {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![0, 5, 5, 0],
                    vec![5, 5, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 5, 0, 0],
                    vec![0, 5, 5, 0],
                    vec![0, 0, 5, 0],
                    vec![0, 0, 0, 0],
                ]
            ],
            x: 4,
            y: 0,
            current_state: 0
        }
    }
}

struct TetriminoZ;

impl TetriminoGenerator for TetriminoZ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![6, 6, 0, 0],
                    vec![0, 6, 6, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 0, 6, 0],
                    vec![0, 6, 6, 0],
                    vec![0, 6, 0, 0],
                    vec![0, 0, 0, 0],
                ]
            ],
            x: 4,
            y: 0,
            current_state: 0
        }
    }
}

struct TetriminoT;

impl TetriminoGenerator for TetriminoT {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![7, 7, 7, 0],
                    vec![0, 7, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 7, 0, 0],
                    vec![7, 7, 0, 0],
                    vec![0, 7, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 7, 0, 0],
                    vec![7, 7, 7, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 7, 0, 0],
                    vec![0, 7, 7, 0],
                    vec![0, 7, 0, 0],
                    vec![0, 0, 0, 0],
                ]
            ],
            x: 4,
            y: 0,
            current_state: 0
        }
    }
}

const TEXTURE_SIZE: u32 = 32;

impl Tetrimino {
    fn rotate(&mut self, game_map: &[Vec<u8>]) {
        let mut tmp_state = self.current_state + 1;

        if tmp_state as usize >= self.states.len() {
            tmp_state = 0;
        }

        let x_pos = [0, -1, 1, -2, 2, -3];
        for x in x_pos.iter() {
            if self.test_position(
                game_map, 
                tmp_state as usize,
                self.x + x, 
                self.y) == true {
                    self.current_state = tmp_state;
                    self.x += *x;
                    break
            }
        }
    }

    fn test_position(
        &self, 
        game_map: &[Vec<u8>], 
        tmp_state: usize, 
        x: isize, 
        y: usize
    ) -> bool {
        for decal_y in 0..4 {
            for decal_x in 0..4 {
                let x = x + decal_x;
    
                if self.states[tmp_state][decal_y][decal_x as usize] != 0
                    &&
                    (
                        y + decal_y >= game_map.len() ||
                        x < 0 ||
                        x as usize >= game_map[y + decal_y].len() ||
                        game_map[y + decal_y][x as usize] != 0
                    ) {
                        return false;
                    }
            }
        }
    
        return true;
    }

    fn change_position(
        &mut self,
        game_map: &[Vec<u8>],
        new_x: isize,
        new_y: usize
    ) -> bool {
        if self.test_position(game_map, self.current_state as usize, new_x, new_y) == true {
            self.x = new_x as isize;
            self.y = new_y;
            true
        } else {
            false
        }
    }

    fn test_current_position(&self, game_map: &[Vec<u8>]) -> bool {
        self.test_position(game_map, self.current_state as usize, self.x, self.y)
    }
}

struct Tetris {
    game_map: Vec<Vec<u8>>,
    current_level: u32,
    score: u32,
    nb_lines: u32,
    current_piece: Option<Tetrimino>,
}

impl Tetris {
    fn new() -> Tetris {
        let mut game_map = Vec::new();

        for _ in 0..16 {
            game_map.push(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }

        Tetris {
            game_map: game_map,
            current_level: 1,
            score: 0,
            nb_lines: 0,
            current_piece: None,
        }
    }

    fn check_lines(&mut self) {
        let mut y = 0;
        let mut score_add = 0;

        while y < self.game_map.len() {
            let mut complete = true;

            for x in &self.game_map[y] {
                if *x == 0 {
                    complete = false;
                    break
                }
            }

            if complete == true {
                score_add += self.current_level;
                self.game_map.remove(y);
                y -= 1;
            }
            y += 1;
        }

        if self.game_map.len() == 0 {
            score_add += 1000;
        }

        while self.game_map.len() < 16 {
            self.increase_line();
            self.game_map.insert(0, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
    }

    fn make_permanent(&mut self) {
        let mut to_add = 0;
        
        if let Some(ref mut piece) = self.current_piece {
            let mut shift_y = 0;

            while shift_y < piece.states[piece.current_state as usize].len() && piece.y + shift_y < self.game_map.len() {
                let mut shift_x = 0;

                while shift_x < piece.states[piece.current_state as usize][shift_y].len() 
                    && (piece.x + shift_x as isize) < self.game_map[piece.y + shift_y].len() as isize 
                {
                    if piece.states[piece.current_state as usize][shift_y][shift_x] != 0 {
                        let x = piece.x + shift_x as isize;
                        self.game_map[piece.y + shift_y][x as usize] = 
                            piece.states[piece.current_state as usize][shift_y][shift_x];
                    }
                    shift_x += 1;
                }
                shift_y += 1;
            }
        }

        self.check_lines();
        self.current_piece = None;
    }

    fn create_new_tetrimino(&self,) -> Tetrimino {
        static mut PREV: u8 = 7;
        let mut rand_nb = rand::random::<u8>() % 7;
    
        if unsafe { PREV } == rand_nb {
            rand_nb = rand::random::<u8>() % 7;
        }
    
        unsafe { PREV = rand_nb; }
    
        match rand_nb {
            0 => TetriminoI::new(),
            1 => TetriminoJ::new(),
            2 => TetriminoL::new(),
            3 => TetriminoO::new(),
            4 => TetriminoS::new(),
            5 => TetriminoZ::new(),
            6 => TetriminoT::new(),
            _ => unreachable!(),
        }
    }

    fn update_score(&mut self, to_add: u32) {
        self.score += to_add;
    }
}

fn handle_events(
    tetris: &mut Tetris, 
    quit: &mut bool, 
    timer: &mut SystemTime, 
    event_pump: &mut sdl2::EventPump
) -> bool {
    let mut make_permanent = false;
    if let Some(ref mut piece) = tetris.current_piece {
        let mut tmp_x = piece.x;
        let mut tmp_y = piece.y;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    *quit = true;
                    break
                }
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    *timer = SystemTime::now();
                    tmp_y += 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                   tmp_x += 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    tmp_x -= 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    piece.rotate(&tetris.game_map);
                }
                Event::KeyDown { keycode: Some(Keycode::Space), ..} => {
                    let x = piece.x;
                    let mut y = piece.y;
                    while piece.change_position(&tetris.game_map, x, y + 1) == true {
                        y += 1;
                    }

                    make_permanent = true;
                }
                _ => {}
            }
        }

        if !make_permanent {
            if piece.change_position(&tetris.game_map, tmp_x, tmp_y) == false && tmp_y != piece.y {
                make_permanent = true;
            }
        }
    }

    if make_permanent {
        tetris.make_permanent();
        *timer = SystemTime::now();
    }

    make_permanent
}

fn print_game_information(tetris: &Tetris) {
    println!("Game over...");
    println!("Score:         {}", tetris.score);
    println!("Current level: {}", tetris.current_level);
}

// #[derive(Clone, Copy)]
// enum TextureColor {
//     Green,
//     Blue,
// }


fn write_into_file(content: &str, file_name: &str) -> io::Result<()> {
    let mut f = File::create(file_name)?;
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
        &format!("{}\n{}\n", s_highscores, s_number_of_lines),
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

fn main() {
    let mut tetris = Tetris::new();
    let mut timer = SystemTime::now();

    let sdl_context = sdl2::init()
        .expect("SDL initialization failed");

    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");


    loop {
        if match timer.elapsed() {
            Ok(elapsed) => elapsed.as_secs() >= 1,
            Err(_) => false,
        } {
            let mut make_permanent = false;
           
            if let Some(ref mut piece) = tetris.current_piece {
                let x = piece.x;
                let y = piece.y + 1;

                make_permanent = !piece.change_position(&tetris.game_map, x, y);
            }

            if make_permanent {
                tetris.make_permanent()
            }

            timer = SystemTime::now();
        }

        if tetris.current_piece.is_none() {
            let current_piece = tetris.create_new_tetrimino();

            if !current_piece.test_current_position(&tetris.game_map) {
                print_game_information(&tetris);
                break
            }

            tetris.current_piece = Some(current_piece);
        }

        let mut quit = false;
        if !handle_events(&mut tetris, &mut quit, &mut timer, &mut event_pump) {
            if let Some(ref mut piece) = tetris.current_piece {

            }
        }

        if quit {
            print_game_information(&tetris);
            break
        }

        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    
    
    
    // let video_subsystem = sdl_context.video()
    // .expect("Couldn't get SDL video subsystem");
    
    // let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
    // .position_centered()
    // .opengl()
    // .build()
    // .expect("Failed to create window");
    
    // let mut canvas = window.into_canvas()
    // .target_texture()
    // .present_vsync()
    // .build()
    // .expect("Failed to convert window into canvas");
    
    // let texture_creator: TextureCreator<_> = canvas.texture_creator();
    
    // sdl2::image::init(InitFlag::PNG | InitFlag::JPG)
    //     .expect("Couldn't initialize image context");

    // let image_texture = texture_creator.load_texture("assets/my_image.png")
    //     .expect("Couldn't load image");

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

    // 'running: loop {
    //     canvas.set_draw_color(Color::RGB(125, 125, 125));
    //     canvas.clear();

    //     canvas.copy(
    //         &image_texture,
    //         None,
    //         None
    //     ).expect("Render failed");
        

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

    //     canvas.present();
        
    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit { .. } |
    //             Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
    //                 break 'running
    //             },
    //             _ => {}
    //         }
    //         sleep(Duration::new(0, 1_000_000_000u32 / 60));
    //     }
    // }
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