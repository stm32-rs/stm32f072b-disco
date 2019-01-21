#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use stm32f072b_disco as board;

use board::hal::{
    gpio::*,
    prelude::*,
    serial::{Event, Serial},
    stm32::{self, interrupt, Interrupt::USART1},
};

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;

use core::{cell::RefCell, fmt::Write, ops::DerefMut};

// Make some peripherals globally available
struct Shared {
    serial: Serial<stm32::USART1, gpioa::PA9<Alternate<AF1>>, gpioa::PA10<Alternate<AF1>>>,
}

static SHARED: Mutex<RefCell<Option<Shared>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let (Some(mut p), Some(cp)) = (stm32::Peripherals::take(), cortex_m::Peripherals::take()) {
        cortex_m::interrupt::free(|cs| {
            let mut rcc = p.RCC.configure().sysclk(48.mhz()).freeze(&mut p.FLASH);
            let gpioa = p.GPIOA.split(&mut rcc);
            let mut nvic = cp.NVIC;

            // USART1 at PA9 (TX) and PA10 (RX) is connectet to ST-Link (well, not quite)
            let tx = gpioa.pa9.into_alternate_af1(cs);
            let rx = gpioa.pa10.into_alternate_af1(cs);

            // Set up serial port
            let mut serial = Serial::usart1(p.USART1, (tx, rx), 115200.bps(), &mut rcc);

            // Enable interrupt generation for received data
            serial.listen(Event::Rxne);

            // Output a nice message
            serial
                .write_str("\r\nTry typing some characters and watch them being echoed.\r\n")
                .ok();

            // Move all components under Mutex supervision
            *SHARED.borrow(cs).borrow_mut() = Some(Shared { serial });

            // Enable USART IRQ and clear any pending IRQs
            nvic.enable(USART1);
            cortex_m::peripheral::NVIC::unpend(USART1);
        });
    }

    loop {
        // Power down a bit while waiting for interrupts
        cortex_m::asm::wfi();
    }
}

// The IRQ handler triggered by a received character in USART buffer
#[interrupt]
fn USART1() {
    cortex_m::interrupt::free(|cs| {
        // Obtain all Mutex protected resources
        if let Some(ref mut shared) = SHARED.borrow(cs).borrow_mut().deref_mut() {
            let serial = &mut shared.serial;

            // Read received character
            let received = serial.read().unwrap();

            // Write character back
            serial.write(received).ok();

            // Clear interrupt
            cortex_m::peripheral::NVIC::unpend(USART1);
        }
    });
}
