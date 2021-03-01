/* 
    Debug helper
    This concept is silly but something I've always wanted to do.
    Write a debug helper which outputs morse code, so you don't need a monitor :)
*/
#![allow(dead_code)]
use crate::board::gpio;
use crate::board::timer::wait_ms;
use crate::sys::lists::{Stack};

// Configure the gpio pin which is used as data output
const GPIO_PIN: u8 = 21;
const SLEEP_TIME_MS: u32 = 350;

const DOT_TIME: u32 = 1;
const WHACK_TIME: u32 = DOT_TIME * 3;
const LETTER_REST_TIME: u32 = DOT_TIME * 3;
const WORD_REST_TIME: u32 = (DOT_TIME * 7) - LETTER_REST_TIME;

pub fn emit(input: &[u8]) {
    for c in input {
        emit_char(*c as char);
        emit_rest();
    }
    emit_word_rest();
}

pub fn emit_num(input: u32) {
    let mut temp = input;
    let mut stack = Stack::new();

    while temp > 0 {
        let v: u8 = (temp % 10) as u8;
        stack.push(v);
        temp /= 10;
    }

    loop {
        match stack.pop() {
            None => break,
            Some(element) => {
                emit_char(num_to_char(element));
                emit_rest();
            },
        }
    }
}

pub fn emit_char(character: char) {
    match character {
        'a' | 'A' => { emit_dot(1); emit_whack(1); },
        'b' | 'B' => { emit_whack(1); emit_dot(3); },
        'c' | 'C' => { emit_whack(1); emit_dot(1); emit_whack(1); emit_dot(1); },
        'd' | 'D' => { emit_whack(1); emit_dot(2); },
        'e' | 'E' => { emit_dot(1); },
        'f' | 'F' => { emit_dot(2); emit_whack(1); emit_dot(1); },
        'g' | 'G' => { emit_whack(2); emit_dot(1); },
        'h' | 'H' => { emit_dot(4); },
        'i' | 'I' => { emit_dot(2); },
        'j' | 'J' => { emit_dot(1); emit_whack(3); },
        'k' | 'K' => { emit_whack(1); emit_dot(1); emit_whack(1); },
        'l' | 'L' => { emit_dot(1); emit_whack(1); emit_dot(2); },
        'm' | 'M' => { emit_whack(2); },
        'n' | 'N' => { emit_whack(1); emit_dot(1); },
        'o' | 'O' => { emit_whack(3); },
        'p' | 'P' => { emit_dot(1); emit_whack(2); emit_dot(1); },
        'q' | 'Q' => { emit_whack(2); emit_dot(1); emit_whack(1); },
        'r' | 'R' => { emit_dot(1); emit_whack(1); emit_dot(1); },
        's' | 'S' => { emit_dot(3); },
        't' | 'T' => { emit_whack(1); },
        'u' | 'U' => { emit_dot(2); emit_whack(1); },
        'v' | 'V' => { emit_dot(3); emit_whack(1); },
        'w' | 'W' => { emit_dot(1); emit_whack(2); },
        'x' | 'X' => { emit_whack(1); emit_dot(2); emit_whack(1); },
        'y' | 'Y' => { emit_whack(1); emit_dot(1); emit_whack(2); },
        'z' | 'Z' => { emit_whack(2); emit_dot(2); },
        
        '1' => { emit_dot(1); emit_whack(4); },
        '2' => { emit_dot(2); emit_whack(3); },
        '3' => { emit_dot(3); emit_whack(2); },
        '4' => { emit_dot(4); emit_whack(1); },
        '5' => { emit_dot(5); },
        '6' => { emit_whack(1); emit_dot(4); }
        '7' => { emit_whack(2); emit_dot(3); }
        '8' => { emit_whack(3); emit_dot(2); }
        '9' => { emit_whack(4); emit_dot(1); }
        '0' => { emit_whack(5); }

        ' ' => { emit_word_rest(); }
        _ => { unsafe { asm!("nop"); } },
    }
}

fn emit_dot(repetition: usize) {
    for _ in 0 .. repetition {
        gpio::set(GPIO_PIN, true);
        wait_ms(SLEEP_TIME_MS * DOT_TIME);
        gpio::set(GPIO_PIN, false);
        wait_ms(SLEEP_TIME_MS * DOT_TIME);
    }
}

fn emit_whack(repition: usize) {
    for _ in 0 .. repition {
        gpio::set(GPIO_PIN, true);
        wait_ms(SLEEP_TIME_MS * WHACK_TIME);
        gpio::set(GPIO_PIN, false);
        wait_ms(SLEEP_TIME_MS * DOT_TIME);
    }
}

fn emit_rest() {
    gpio::set(GPIO_PIN, false);
    wait_ms(SLEEP_TIME_MS * LETTER_REST_TIME);
}

fn emit_word_rest() {
    gpio::set(GPIO_PIN, false);
    wait_ms(SLEEP_TIME_MS * WORD_REST_TIME);
}

fn num_to_char(input: u8) -> char {
    if input >= 10 {
        return '\0';
    } else {
        return (('0' as u8) + input) as char;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number_to_char() {
        assert_eq!(num_to_char(0), '0');
        assert_eq!(num_to_char(1), '1');
    }

    #[test]
    fn test_emit_num() {
        let mut temp = 102;
        assert_eq!((temp % 10) as u8, 2);
        temp /= 10;
        assert_eq!((temp % 10) as u8, 0);
        temp /= 10;
        assert_eq!((temp % 10) as u8, 1);
    }
}