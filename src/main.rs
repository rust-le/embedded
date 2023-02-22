// std and main are not available for bare metal software
#![no_std]
#![no_main]
// https://jonathanklimt.de/electronics/programming/embedded-rust/rust-on-stm32-2/
use core::fmt::Write;
use core::str::from_utf8;
use cortex_m_rt::entry; // The runtime
use embedded_hal::digital::v2::OutputPin; // the `set_high/low`function
use max7219_dot_matrix::{Command, MAX7219};
use nb::block;

#[allow(unused_imports)]
use panic_halt;
use stm32f1xx_hal::{
    delay::Delay,
    pac,
    prelude::*,
    serial::{Config, Serial},
};

// STM32F1 specific functions // When a panic occurs, stop the microcontroller
#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let mut output_a = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);
    let mut output_b = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);
    let mut output_c = gpioa.pa3.into_push_pull_output(&mut gpioa.crl);
    let mut output_d = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);

    //let clocks = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr);
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut delay = Delay::new(cp.SYST, clocks);

    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    let rx = gpiob.pb11;

    let mut serial = Serial::usart3(
        dp.USART3,
        (tx, rx),
        &mut afio.mapr,
        Config::default().baudrate(9600.bps()),
        clocks,
        &mut rcc.apb1,
    );
    let (mut tx, mut rx) = serial.split();

    loop {
        block!(tx.write(b'R')).ok();
        let received = block!(rx.read()).unwrap();
        delay.delay_ms(100_u16);
        led.set_high().ok();
        delay.delay_ms(100_u16);
        led.set_low().ok();
    }
    /* let half_step_clockwise = [
        [1, 0, 0, 0],
        [1, 1, 0, 0],
        [0, 1, 0, 0],
        [0, 1, 1, 0],
        [0, 0, 1, 0],
        [0, 0, 1, 1],
        [0, 0, 0, 1],
        [1, 0, 0, 1],
    ];*/
    /*let mut rotate_clockwise = ||{
        for _ in 0..31 {
            half_step_clockwise.iter().for_each(|tick| {
                match tick[0] {
                    0 => output_a.set_low().ok(),
                    _ => output_a.set_high().ok(),
                };
                match tick[1] {
                    0 => output_b.set_low().ok(),
                    _ => output_b.set_high().ok(),
                };
                match tick[2] {
                    0 => output_c.set_low().ok(),
                    _ => output_c.set_high().ok(),
                };
                match tick[3] {
                    0 => output_d.set_low().ok(),
                    _ => output_d.set_high().ok(),
                };
                delay.delay_ms(50_u16);
            });
        }
        output_a.set_low().ok();
        output_b.set_low().ok();
        output_c.set_low().ok();
        output_d.set_low().ok();
    };*/
    /*loop {
        // rotate_clockwise();
        delay.delay_ms(1000_u16);
        led.set_high().ok();
        delay.delay_ms(200_u16);
        led.set_low().ok();
    }*/
}
