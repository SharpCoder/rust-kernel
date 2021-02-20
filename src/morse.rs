/* 
    Debug helper
    This concept is silly but something I've always wanted to do.
    Write a debug helper which outputs morse code, so you don't need a monitor :)
*/
use lib;
use gpio;

// Configure the gpio pin which is used as data output
const GPIO_PIN: u8 = 21;
const SLEEP_TIME: u32 = 1000000; // 750000

const DOT_TIME: u32 = 3;
const WHACK_TIME: u32 = DOT_TIME * 3;
const LETTER_REST_TIME: u32 = DOT_TIME * 3;
const WORD_REST_TIME: u32 = DOT_TIME * 4;

pub fn emit(input: &str) {
    for c in input.bytes() {
        emit_char(c as char);
        emit_rest();
    }
    
    emit_word_rest();
}

fn emit_char(character: char) {
    match character {
        'a' | 'A' => { emit_dot(); emit_whack(); },
        'b' | 'B' => { emit_whack(); emit_dot(); emit_dot(); emit_dot(); },
        'c' | 'C' => { emit_whack(); emit_dot(); emit_whack(); emit_dot(); },
        'd' | 'D' => { emit_whack(); emit_dot(); emit_dot(); },
        'e' | 'E' => { emit_dot(); },
        'f' | 'F' => { emit_dot(); emit_dot(); emit_whack(); emit_dot(); },
        'g' | 'G' => { emit_whack(); emit_whack(); emit_dot(); },
        'h' | 'H' => { emit_dot(); emit_dot(); emit_dot(); emit_dot(); },
        'i' | 'I' => { emit_dot(); emit_dot(); },
        'j' | 'J' => { emit_dot(); emit_whack(); emit_whack(); emit_whack(); },
        'k' | 'K' => { emit_whack(); emit_dot(); emit_whack(); },
        'l' | 'L' => { emit_dot(); emit_whack(); emit_dot(); emit_dot(); },
        'm' | 'M' => { emit_whack(); emit_whack(); },
        'n' | 'N' => { emit_whack(); emit_dot(); },
        'o' | 'O' => { emit_whack(); emit_whack(); emit_whack(); },
        'p' | 'P' => { emit_dot(); emit_whack(); emit_whack(); emit_dot(); },
        'q' | 'Q' => { emit_whack(); emit_whack(); emit_dot(); emit_whack(); },
        'r' | 'R' => { emit_dot(); emit_whack(); emit_dot(); },
        's' | 'S' => { emit_dot(); emit_dot(); emit_dot(); },
        't' | 'T' => { emit_whack(); },
        'u' | 'U' => { emit_dot(); emit_dot(); emit_whack(); },
        'v' | 'V' => { emit_dot(); emit_dot(); emit_dot(); emit_whack(); },
        'w' | 'W' => { emit_dot(); emit_whack(); emit_whack(); },
        'x' | 'X' => { emit_whack(); emit_dot(); emit_dot(); emit_whack(); },
        'y' | 'Y' => { emit_whack(); emit_dot(); emit_whack(); emit_whack(); },
        'z' | 'Z' => { emit_whack(); emit_whack(); emit_dot(); emit_dot(); },
        
        '1' => { emit_dot(); emit_whack(); emit_whack(); emit_whack(); emit_whack(); },
        '2' => { emit_dot(); emit_dot(); emit_whack(); emit_whack(); emit_whack(); },
        '3' => { emit_dot(); emit_dot(); emit_dot(); emit_whack(); emit_whack(); },
        '4' => { emit_dot(); emit_dot(); emit_dot(); emit_dot(); emit_whack(); },
        '5' => { emit_dot(); emit_dot(); emit_dot(); emit_dot(); emit_dot(); },
        '6' => { emit_whack(); emit_dot(); emit_dot(); emit_dot(); emit_dot(); }
        '7' => { emit_whack(); emit_whack(); emit_dot(); emit_dot(); emit_dot(); }
        '8' => { emit_whack(); emit_whack(); emit_whack(); emit_dot(); emit_dot(); }
        '9' => { emit_whack(); emit_whack(); emit_whack(); emit_whack(); emit_dot(); }
        '0' => { emit_whack(); emit_whack(); emit_whack(); emit_whack(); emit_whack(); }

        ' ' => { emit_word_rest(); }
        _ => { },
    }
}

fn emit_dot() {
    gpio::set(GPIO_PIN, true);
    lib::sleep(SLEEP_TIME * DOT_TIME);
    gpio::set(GPIO_PIN, false);
    lib::sleep(SLEEP_TIME * DOT_TIME);
}

fn emit_whack() {
    gpio::set(GPIO_PIN, true);
    lib::sleep(SLEEP_TIME * WHACK_TIME);
    gpio::set(GPIO_PIN, false);
    lib::sleep(SLEEP_TIME * DOT_TIME);
}

fn emit_rest() {
    gpio::set(GPIO_PIN, false);
    lib::sleep(SLEEP_TIME * LETTER_REST_TIME);
}

fn emit_word_rest() {
    gpio::set(GPIO_PIN, false);
    lib::sleep(SLEEP_TIME * WORD_REST_TIME);
}