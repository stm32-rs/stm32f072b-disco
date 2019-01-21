#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use stm32f072b_disco as board;

use board::hal::{prelude::*, serial::Serial, stm32};

use cortex_m_rt::entry;
use nb::block;
use core::fmt::Write;

#[entry]
fn main() -> ! {
    if let Some(mut p) = stm32::Peripherals::take() {
        cortex_m::interrupt::free(|cs| {
            let mut rcc = p.RCC.configure().sysclk(48.mhz()).freeze(&mut p.FLASH);
            let gpioa = p.GPIOA.split(&mut rcc);

            // USART1 at PA9 (TX) and PA10 (RX) is connectet to ST-Link (well, not quite)
            let tx = gpioa.pa9.into_alternate_af1(cs);
            let rx = gpioa.pa10.into_alternate_af1(cs);

            // Set up serial port
            let mut serial = Serial::usart1(p.USART1, (tx, rx), 115200.bps(), &mut rcc);

            serial.write_str("Type for echo!\n\r").ok();

            loop {
                let received = block!(serial.read()).unwrap();
                block!(serial.write(received)).ok();
            }
        });
    }

    loop {
        continue;
    }
}
