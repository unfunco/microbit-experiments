#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use microbit::{display::blocking::Display, hal::Timer, Board};

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {
        let mut display = Display::new(board.display_pins);
        let mut timer = Timer::new(board.TIMER0);

        let penis = [
            [0, 1, 1, 1, 0],
            [0, 1, 0, 1, 0],
            [1, 1, 0, 1, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
        ];

        loop {
            display.show(&mut timer, penis, 1000);
            display.clear();
            timer.delay_ms(250_u32);
        }
    }

    panic!("End");
}
