use crate::structures;

#[allow(unused_variables)]
pub fn handle_char_pos(char_pos: structures::Position, term_width: usize, term_height: usize) -> structures::Position {
    char_pos
}
pub fn handle_character_movement(
        mut char_pos: structures::Position,
        term_width: u16,
        term_height: u16,
        input: u8) -> 
structures::Position 
{   
    char_pos = match char_pos {
        structures::Position(a,b) if (a>=1 && a<=term_width-3)
            && ( b>=1 && b<=term_height-2) => structures::Position(a,b),
        _ => {
            char_pos.set(term_width, term_height)
        }
    };
    match input {
        100 | 68 => {
            // D or d is pressed
            if char_pos.0 < term_width-3 {
                char_pos.0 += 1;
            }
        }, 
        83 | 115 => {
            // S or s is pressed
            if char_pos.1 < term_height-2 {
                char_pos.1 += 1;
            }
        }, 
        87 | 119 => {
            // W or w is pressed
            if char_pos.1 > 1 {
                char_pos.1 -= 1;
            }
        },
        97 | 65 => {
            // A or a is pressed
            if char_pos.0 > 1 {
                char_pos.0 -= 1;
            }
        },
        _error => ()
    }
    char_pos
}
