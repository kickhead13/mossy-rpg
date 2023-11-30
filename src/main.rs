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

        fps_contor += 1;
        stdout.queue(cursor::MoveTo(2,0)).unwrap();
        stdout.write(format!("{}",fps_contor).as_bytes()).unwrap();

        stdout.queue(cursor::MoveTo(char_pos.0, char_pos.1)).unwrap();
        stdout.write("  ".as_bytes()).unwrap();

       // println!("main thread");

        if let Ok(input) = receiver.try_recv() {
           match input {
                100 | 68 => char_pos.0 += 1,
                83 | 115 => char_pos.1 +=1,
                87 | 119 => {
                        if char_pos.1 > 1 {
                            char_pos.1 -= 1;
                        }
                    },
                97 | 65 => {
                    
                        if char_pos.0 > 0 {
                            char_pos.0 -= 1;
                        }
                    },
                Error => ()
            }
        } 
        else {
        
         //   println!("received");

            stdout.queue(cursor::MoveTo(char_pos.0, char_pos.1)).unwrap();
            stdout.write(":3".as_bytes()).unwrap();

            (term_width, term_height) = terminal::size().unwrap();

            stdout.queue(cursor::MoveTo(0,0)).unwrap();
            stdout.write("a".as_bytes()).unwrap();
    

            stdout.queue(cursor::MoveTo(term_width, term_height)).unwrap();
            stdout.write("b".as_bytes()).unwrap();
            stdout.flush();

            std::thread::sleep(Duration::from_millis(100));
        }
    }
    std::thread::sleep(Duration::from_secs(5));

    stdout.queue(cursor::MoveTo(7,5)).unwrap();
    stdout.write("lol".as_bytes()).unwrap();

       

}


//W = 87
//A = 97
//S = 83
//D = 68
//
//w = 119
//a = 97
//s = 115
//d = 100
