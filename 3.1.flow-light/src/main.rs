#![no_std]
#![no_main]

use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal::{clocks, pac, Clock, Sio, Watchdog};

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = Sio::new(pac.SIO);
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut leds = [
        pins.gpio6.into_push_pull_output().into_dyn_pin(),
        pins.gpio7.into_push_pull_output().into_dyn_pin(),
        pins.gpio8.into_push_pull_output().into_dyn_pin(),
        pins.gpio9.into_push_pull_output().into_dyn_pin(),
        pins.gpio10.into_push_pull_output().into_dyn_pin(),
        pins.gpio11.into_push_pull_output().into_dyn_pin(),
        pins.gpio12.into_push_pull_output().into_dyn_pin(),
        pins.gpio13.into_push_pull_output().into_dyn_pin(),
        pins.gpio14.into_push_pull_output().into_dyn_pin(),
        pins.gpio15.into_push_pull_output().into_dyn_pin(),
    ];

    loop {
        for led in leds.iter_mut().rev() {
            led.set_high().unwrap();
            delay.delay_ms(100);
            led.set_low().unwrap();
        }
        for led in leds.iter_mut() {
            led.set_high().unwrap();
            delay.delay_ms(100);
            led.set_low().unwrap();
        }
    }
}
