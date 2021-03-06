extern crate rand;
extern crate rustbox;
extern crate fps_clock;
extern crate snake;
use std::default::Default;
use std::time::Duration;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use rustbox::{Color, RustBox, Key};
use rand::Rng;
use snake::{direction, point, player, apple};
use fps_clock::FpsClock;

fn show_intro(rb: &RustBox) {
    let ascii_art = vec![
    " ____              _        _",
    "/ ___| _ __   __ _| | _____| |",
    "\\___ \\| '_ \\ / _` | |/ / _ \\ |",
    " ___) | | | | (_| |   <  __/_|",
    "|____/|_| |_|\\__,_|_|\\_\\___(_)"];
    let art_width   = ascii_art[0].len() as i16;
    let art_height  = ascii_art.len() as i16;
    let term_width  = rb.width() as i16;
    let term_height = rb.height() as i16;
    let start_x = ((term_width - art_width) / 2) as i16;
    let start_y = ((term_height - art_height) / 2) as i16;
    for (i, y) in (start_y..(start_y + art_height)).enumerate() {
        rb.print(start_x as usize, y as usize, rustbox::RB_BOLD, Color::Green, Color::Black, ascii_art[i]);
    };
    let next_key_message = "Hit space to continue.";
    let quit_message = "Hit <esc> to exit.";
    rb.print(((term_width - next_key_message.len() as i16)/2) as usize, (term_height - 2) as usize,
             rustbox::RB_BOLD, Color::White, Color::Black, next_key_message);
    rb.print(((term_width - quit_message.len() as i16)/2) as usize, (term_height - 1) as usize,
             rustbox::RB_BOLD, Color::White, Color::Black, quit_message);
    loop {
        rb.present();
        match rb.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char(' ') => { break; },
                    Key::Esc => { panic!(); },
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e),
            _ => { }
        }
    }
}

fn draw_playfield(rb: &RustBox, start: point::Point, width: i16, height: i16) {
    // Draw the four corners
    rb.print(start.x as usize, start.y as usize, rustbox::RB_NORMAL,
             Color::White, Color::Black, "╔");
    rb.print((start.x + width) as usize, start.y as usize, rustbox::RB_NORMAL,
             Color::White, Color::Black, "╗");
    rb.print(start.x as usize, (start.y + height) as usize, rustbox::RB_NORMAL,
             Color::White, Color::Black, "╚");
    rb.print((start.x + width) as usize, (start.y + height) as usize, rustbox::RB_NORMAL,
             Color::White, Color::Black, "╝");
    for x in (start.x + 1)..width {
        rb.print(x as usize, start.y as usize, rustbox::RB_NORMAL,
                 Color::White, Color::Black, "═");
        rb.print(x as usize, (start.y + height) as usize, rustbox::RB_NORMAL,
                 Color::White, Color::Black, "═");
    }
    for y in (start.y + 1)..(height + 1) {
        rb.print(start.x as usize, y as usize, rustbox::RB_NORMAL,
                 Color::White, Color::Black, "║");
        rb.print((start.x + width) as usize, y as usize, rustbox::RB_NORMAL,
                 Color::White, Color::Black, "║");
    }
}

fn draw_score(rb: &RustBox, score: u32, highscore: u32) {
    rb.print(0, 0, rustbox::RB_NORMAL, Color::White, Color::Black,
        &format!("Score: {} Highscore: {}", score, highscore));
}

fn draw_snake(rb: &RustBox, snake: &player::Snake) {
    rb.print_char(snake.head.x as usize, snake.head.y as usize,
                  rustbox::RB_BOLD, Color::Green, Color::Black,
                  '█');
    for segment in &snake.body {
        rb.print_char(segment.x as usize, segment.y as usize,
                      rustbox::RB_BOLD, Color::Green, Color::Black,
                      '▒');
    }
}

fn draw_apple(rb: &RustBox, apple: &apple::Apple) {
    rb.print_char(apple.position.x as usize, apple.position.y as usize,
                  rustbox::RB_BOLD, Color::Red, Color::Black,
                  apple::apple_char());
}

fn game_over(rb: &RustBox) {
    //rb.clear();
    let ascii_art = vec![
    "  ____                         ___                 _",
    " / ___| __ _ _ __ ___   ___   / _ \\__   _____ _ __| |",
    "| |  _ / _` | '_ ` _ \\ / _ \\ | | | \\ \\ / / _ \\ '__| |",
    "| |_| | (_| | | | | | |  __/ | |_| |\\ V /  __/ |  |_|",
    " \\____|\\__,_|_| |_| |_|\\___|  \\___/  \\_/ \\___|_|  (_)"
    ];
    let art_width   = ascii_art[0].len() as i16;
    let art_height  = ascii_art.len() as i16;
    let term_width  = rb.width() as i16;
    let term_height = rb.height() as i16;
    let start_x     = ((term_width - art_width) / 2) as i16;
    let start_y     = ((term_height - art_height) / 2) as i16;
    for (i, y) in (start_y..(start_y + art_height)).enumerate() {
        rb.print(start_x as usize, y as usize, rustbox::RB_BOLD, Color::Red, Color::Black, ascii_art[i]);
    };
    let quit_message = "Hit <esc> to exit.";
    rb.print(((term_width - quit_message.len() as i16)/2) as usize, (term_height - 1) as usize,
             rustbox::RB_BOLD, Color::White, Color::Black, quit_message);
    loop {
        rb.present();
        match rb.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Esc => { panic!(); },
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e),
            _ => { }
        }
    }
}

fn get_highscore_path() -> String {
        format!("{}/{}", env::home_dir().unwrap().display(), ".snake_highscore")
}

fn get_highscore(path: &str) -> Result<u32, io::Error> {
    let mut f = File::open(path)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf.parse().unwrap())
}

fn write_highscore(path: &str, score: u32) -> io::Result<()> {
    let mut f = File::create(path)?;
    write!(f, "{:?}", score)?;
    Ok(())
}

fn main() {
    //let log = File::create("log").unwrap();
    let highscore_file = get_highscore_path();
    let rb = match RustBox::init(Default::default()) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    show_intro(&rb);
    rb.clear();
    let mut score = 0;
    let mut highscore = get_highscore(&highscore_file).unwrap_or(0);
    let mut move_counter = 0;
    let frames_per_move = 4;
    let mut snake = player::Snake::new(point::Point::random(5, (rb.width() - 5) as i16,
                                                            5, (rb.height() - 5) as i16),
                                       3, rand::thread_rng().gen());
    let mut apple = apple::Apple::new(point::Point::random(5, (rb.width() - 5) as i16,
                                                           5, (rb.height() - 5) as i16));
    // Main game loop
    let mut clock = FpsClock::new(30); // Run at 30 FPS
    loop {
        rb.clear();
        draw_playfield(&rb, point::Point::new(0, 1), (rb.width() - 1) as i16, (rb.height() - 2) as i16);
        // rb.present() MUST be called after all the drawing functions have been called
        draw_score(&rb, score, highscore);
        draw_apple(&rb, &apple);
        draw_snake(&rb, &snake);
        rb.present();
        match rb.peek_event(Duration::new(0, 500), false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Esc   => { break; },
                    Key::Up    => if snake.direction != direction::Direction::Up { snake.direction = direction::Direction::Down; },
                    Key::Down  => if snake.direction != direction::Direction::Down { snake.direction = direction::Direction::Up; },
                    Key::Left  => if snake.direction != direction::Direction::Right { snake.direction = direction::Direction::Left; },
                    Key::Right => if snake.direction != direction::Direction::Left { snake.direction = direction::Direction::Right; },
                    _ => { }
                }
            },
            Err(_) => {
                //log.write(format!("{}\n", e).as_bytes()).unwrap();
                break;
            },
            _ => { }
        };
        if move_counter == frames_per_move {
            move_counter = 0;
            let next_pos = snake.next_position();
            if snake.body.contains(&next_pos) ||
               next_pos.x == 0 || next_pos.x + 1 == rb.width() as i16 ||
               next_pos.y == 1 || next_pos.y + 1 == rb.height() as i16 {
                game_over(&rb);
            } else if next_pos == apple.position {
                snake.move_forward_and_eat();
                apple = apple::Apple::new(point::Point::random(5, (rb.width() - 5) as i16,
                                                               5, (rb.height() - 5) as i16));
                score += 1;
                if score > highscore {
                    highscore = score;
                    match write_highscore(&highscore_file, highscore) {
                        Ok(_) => {},
                        Err(_) => {
                            //log.write(format!("{:?} {}\n", highscore_file, e).as_bytes()).unwrap();
                        }
                    };
                }
            } else {
                snake.move_forward();
            }
        } else {
            move_counter += 1;
        }
        clock.tick();
    }
}
