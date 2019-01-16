#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use stm32f0xx_hal as hal;

use cortex_m_rt::entry;

use crate::hal::{delay::Delay, prelude::*, stm32};

use cortex_m::peripheral::Peripherals;

#[entry]
fn main() -> ! {
    if let (Some(mut p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        cortex_m::interrupt::free(|cs| {
            // Configure clock to 8 MHz (i.e. the default) and freeze it
            let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);

            // Obtain resources from GPIO port C
            let gpioc = p.GPIOC.split(&mut rcc);

            // Initialize on-board LEDs
            let mut orange = gpioc.pc8.into_push_pull_output(cs);
            let mut green = gpioc.pc9.into_push_pull_output(cs);
            let mut red = gpioc.pc6.into_push_pull_output(cs);
            let mut blue = gpioc.pc7.into_push_pull_output(cs);

            // Get delay provider
            let mut delay = Delay::new(cp.SYST, &rcc);

            loop {
                orange.toggle();
                delay.delay_ms(200_u16);
                red.toggle();
                delay.delay_ms(200_u16);
                green.toggle();
                delay.delay_ms(200_u16);
                blue.toggle();
                delay.delay_ms(200_u16);
            }
        });
    }

    loop {
        continue;
    }
}
