#![no_main]
#![no_std]

#[allow(unused)]
use panic_ramdump;

#[allow(unused)]
use stm32f072b_disco as board;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    panic!("Hello world");
}
