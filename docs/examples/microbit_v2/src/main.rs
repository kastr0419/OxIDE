//! micro:bit v2 ボタン + LED ディスプレイ例
//! ボタンA: HEART パターン表示
//! ボタンB: SMILE パターン表示
//! 非押下時: 中央1点灯
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::Timer,
};
use embedded_hal::digital::InputPin;
use panic_halt as _;

const HEART: [[u8; 5]; 5] = [
    [0, 1, 0, 1, 0],
    [1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1],
    [0, 1, 1, 1, 0],
    [0, 0, 1, 0, 0],
];

const SMILE: [[u8; 5]; 5] = [
    [0, 1, 0, 1, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
];

const DOT: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut buttons = board.buttons;

    loop {
        if buttons.button_a.is_low().unwrap() && buttons.button_b.is_low().unwrap() {
            // A+B 同時押し: HEART を 500ms
            display.show(&mut timer, HEART, 500);
        } else if buttons.button_a.is_low().unwrap() {
            // A 押下: HEART を 500ms
            display.show(&mut timer, HEART, 500);
        } else if buttons.button_b.is_low().unwrap() {
            // B 押下: SMILE を 500ms
            display.show(&mut timer, SMILE, 500);
        } else {
            // 非押下: 中央ドット 100ms（応答性確保）
            display.show(&mut timer, DOT, 100);
        }
    }
}
