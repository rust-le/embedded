// std and main are not available for bare metal software
#![no_std]
#![no_main]
// https://jonathanklimt.de/electronics/programming/embedded-rust/rust-on-stm32-2/
use core::fmt::Write;
use core::str::from_utf8;
use cortex_m_rt::entry; // The runtime
use embedded_hal::digital::v2::OutputPin; // the `set_high/low`function
use nb::block;

#[allow(unused_imports)]
use panic_halt;
use stm32f1xx_hal::{
    delay::Delay,
    pac,
    prelude::*,
};

// STM32F1 specific functions // When a panic occurs, stop the microcontroller
#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    //let clocks = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr);
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut delay = Delay::new(cp.SYST, clocks);



    loop {
        delay.delay_ms(100_u16);
        led.set_high().ok();
        delay.delay_ms(100_u16);
        led.set_low().ok();
    }
}
