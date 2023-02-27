#![no_std]
#![no_main]
extern crate panic_semihosting;
use cortex_m::asm::delay;
use cortex_m_rt::entry;
use stm32f1xx_hal::usb::{Peripheral, UsbBus};
use stm32f1xx_hal::{pac, prelude::*};
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(48.MHz())
        .pclk1(24.MHz())
        .freeze(&mut flash.acr);

    assert!(clocks.usbclk_valid());

    let mut gpioc = dp.GPIOC.split();
    let mut gpiob = dp.GPIOB.split();
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let mut green_led = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
    let mut yellow_led = gpiob.pb13.into_push_pull_output(&mut gpiob.crh);
    let mut red_led = gpiob.pb14.into_push_pull_output(&mut gpiob.crh);
    led.set_high(); // Turn off
    green_led.set_low();
    yellow_led.set_low();
    red_led.set_low();
    let mut gpioa = dp.GPIOA.split();

    let mut usb_dp = gpioa.pa12.into_push_pull_output(&mut gpioa.crh);
    usb_dp.set_low();
    delay(clocks.sysclk().raw() / 100);
    let usb = Peripheral {
        usb: dp.USB,
        pin_dm: gpioa.pa11,
        pin_dp: usb_dp.into_floating_input(&mut gpioa.crh),
    };
    let usb_bus = UsbBus::new(usb);
    let mut serial = SerialPort::new(&usb_bus);
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(USB_CLASS_CDC)
        .build();
    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }
        let mut buf = [0u8; 8];
        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                led.set_low();
                for c in buf[0..count].iter_mut() {
                    if *c >= 0x30 && *c <= 0x37 {
                        match *c & (1 << 0) {
                            0 => {
                                red_led.set_low();
                            }
                            _ => {
                                red_led.set_high();
                            }
                        }
                        match *c & (1 << 1) {
                            0 => {
                                yellow_led.set_low();
                            }
                            _ => {
                                yellow_led.set_high();
                            }
                        }
                        match *c & (1 << 2) {
                            0 => {
                                green_led.set_low();
                            }
                            _ => {
                                green_led.set_high();
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        led.set_high();
    }
}
