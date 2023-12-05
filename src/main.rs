mod structures;
use structures::Position;
mod character;

use std::io::{Write, stdout};
use crossterm::{QueueableCommand, cursor, terminal}; 
use std::time::Duration;
use getch::Getch;
use std::sync::mpsc;

#[allow(unused_must_use)]

fn main() {
    let mut is_next_frame = true;

    let character_sprite: &str = ":3";
    let sleep_duration = Duration::from_millis(100);

    let (sender, receiver) = mpsc::channel::<u8>();
    let getter = Getch::new();

    let _input_thread = std::thread::spawn( move || {
        loop {
            if let Ok(input) = getter.getch() {
                sender.send(input as u8);
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
        for x in 1..((term_height-1) as usize) {
            let row = (0..(term_width-2)).map(|_|" ").collect::<String>();
            stdout.queue(cursor::MoveTo(1, x as u16)).unwrap();
            stdout.write(row.as_bytes()).unwrap();
        } 
        fps_contor += 1;
        stdout.queue(cursor::MoveTo(2,0)).unwrap();
        stdout.write(format!("{}",fps_contor).as_bytes()).unwrap();
        if let Ok(input) = receiver.try_recv() {
            if is_next_frame {
                char_pos = character::handle_character_movement(
                    char_pos,
                    term_width,
                    term_height,
                input);
                is_next_frame = false;
            }
        }
        else {
            char_pos = character::handle_character_movement(
                char_pos,
                term_width,
                term_height,
            0);
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


