use std::io::{Write, stdout};
use crossterm::{QueueableCommand, cursor, terminal}; 
use std::time::Duration;
use getch::Getch;
use std::sync::mpsc;

pub struct Position(u16, u16);

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Position(x, y)
    }
}

fn main() {

    let mut is_next_frame = true;

    let character_sprite: &str = ":3";
    let sleep_duration = Duration::from_millis(100);

    let (sender, receiver) = mpsc::channel::<u8>();
    let mut getter = Getch::new();

    let _input_thread = std::thread::spawn( move || {
        loop {
            if let Ok(input) = getter.getch() {
                sender.send(input as u8);
                //std::thread::sleep(sleep_duration);
                //getter = Getch::new();
            }
        }
    });
    
    let mut char_pos: Position = Position::new(1,1);
    let (mut term_width, mut term_height) = terminal::size().unwrap();
    let mut stdout = stdout();
    stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
    let mut fps_contor: u32 = 0;

    loop {
        stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
        fps_contor += 1;
        stdout.queue(cursor::MoveTo(2,0)).unwrap();
        stdout.write(format!("{}",fps_contor).as_bytes()).unwrap();
        if let Ok(input) = receiver.try_recv() {
            if is_next_frame {
                match input {
                    100 | 68 => char_pos.0 += 1, // D or d is pressed
                    83 | 115 => char_pos.1 +=1, // S or s is pressed
                    87 | 119 => {
                        // W or w is pressed
                        if char_pos.1 > 1 {
                            char_pos.1 -= 1;
                        }
                    },
                    97 | 65 => {
                        // A or a is pressed
                        if char_pos.0 > 0 {
                            char_pos.0 -= 1;
                        }
                    },
                    Error => ()
                }
                is_next_frame = false;
            }
        }
        else {
            stdout.queue(cursor::MoveTo(char_pos.0, char_pos.1)).unwrap();
            stdout.write(character_sprite.as_bytes()).unwrap();
            (term_width, term_height) = terminal::size().unwrap();
            stdout.queue(cursor::MoveTo(0,0)).unwrap();
            stdout.write("a".as_bytes()).unwrap();
            stdout.queue(cursor::MoveTo(term_width, term_height)).unwrap();
            stdout.write("b".as_bytes()).unwrap();
            stdout.flush();
            std::thread::sleep(sleep_duration);
            is_next_frame = true;
        }
    }
}
