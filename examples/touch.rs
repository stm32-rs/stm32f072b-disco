#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use stm32f072b_disco as board;

use board::hal::{delay::Delay, prelude::*, serial::Serial, stm32, tsc::Tsc};
use board::{blue, green, orange, red};

use core::fmt::Write;
use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let (Some(mut p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        cortex_m::interrupt::free(|cs| {
            let mut rcc = p.RCC.configure().sysclk(48.mhz()).freeze(&mut p.FLASH);
            let gpioa = p.GPIOA.split(&mut rcc);
            let gpiob = p.GPIOB.split(&mut rcc);

            // USART1 at PA9 (TX) and PA10 (RX) is connectet to ST-Link (well, not quite)
            let tx = gpioa.pa9.into_alternate_af1(cs);
            let rx = gpioa.pa10.into_alternate_af1(cs);

            // Obtain resources from GPIO port C
            let gpioc = p.GPIOC.split(&mut rcc);

            // Initialize on-board LEDs
            let mut orange = orange!(gpioc, cs);
            let mut green = green!(gpioc, cs);
            let mut red = red!(gpioc, cs);
            let mut blue = blue!(gpioc, cs);

            // Set up serial port
            let mut serial = Serial::usart1(p.USART1, (tx, rx), 115200.bps(), &mut rcc);

            // Initialise touch controller
            let mut tsc = Tsc::tsc(p.TSC, &mut rcc, None);

            let mut sample1 = gpioa.pa3.into_alternate_af3(cs).set_open_drain(cs);
            tsc.setup_sample_group(&mut sample1);
            let mut sense1 = gpioa.pa2.into_alternate_af3(cs);
            tsc.enable_channel(&mut sense1);

            let mut sample2 = gpioa.pa7.into_alternate_af3(cs).set_open_drain(cs);
            tsc.setup_sample_group(&mut sample2);
            let mut sense2 = gpioa.pa6.into_alternate_af3(cs);
            tsc.enable_channel(&mut sense2);

            let mut sample3 = gpiob.pb1.into_alternate_af3(cs).set_open_drain(cs);
            tsc.setup_sample_group(&mut sample3);
            let mut sense3 = gpiob.pb0.into_alternate_af3(cs);
            tsc.enable_channel(&mut sense3);

            // Get delay provider
            let mut delay = Delay::new(cp.SYST, &rcc);

            // Aquire a baseline reading
            tsc.acquire().unwrap();

            // Store the baseline values
            let mut base1 = tsc.read(&mut sense1).unwrap();
            writeln!(serial, "base: {}!\r", base1).ok();
            let mut base2 = tsc.read(&mut sense2).unwrap();
            writeln!(serial, "base: {}!\r", base2).ok();
            let mut base3 = tsc.read(&mut sense3).unwrap();
            writeln!(serial, "base: {}!\r", base3).ok();

            loop {
                // Aquire a regular reading
                tsc.acquire().unwrap();

                // Compare the new values with the baseline
                let touched = tsc.read(&mut sense1).unwrap();
                let reading1 = base1 - touched;
                base1 = core::cmp::max(base1, touched);
                writeln!(serial, "sense1: {}!\r", reading1).ok();

                let touched = tsc.read(&mut sense2).unwrap();
                let reading2 = base2 - touched;
                writeln!(serial, "sense2: {}!\r", reading2).ok();
                base2 = core::cmp::max(base2, touched);

                let touched = tsc.read(&mut sense3).unwrap();
                let reading3 = base3 - touched;
                writeln!(serial, "sense3: {}!\r", reading3).ok();
                base3 = core::cmp::max(base3, touched);

                // Light up the LEDs depending on the position
                if reading1 > 100 {
                    green.set_high();
                } else {
                    green.set_low();
                }

                if (reading2) > 100 {
                    blue.set_high();
                } else {
                    blue.set_low();
                }

                if (reading3) > 100 {
                    red.set_high();
                } else {
                    red.set_low();
                }

                if (reading3) > 100 {
                    orange.set_high();
                } else {
                    orange.set_low();
                }

                delay.delay_ms(100_u16);
            }
        });
    }

    loop {
        continue;
    }
}
