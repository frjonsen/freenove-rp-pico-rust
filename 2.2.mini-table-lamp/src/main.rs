#![no_std]
#![no_main]

use core::ops::Not;

use embedded_hal::digital::v2::{InputPin, OutputPin, PinState};
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

    let mut led_pin = pins.gpio15.into_push_pull_output();
    let button_pin = pins.gpio13.into_pull_up_input();

    let mut led_state = PinState::High;
    loop {
        if button_pin.is_low().unwrap() {
            delay.delay_ms(50);
            if button_pin.is_low().unwrap() {
                led_state = led_state.not();
                led_pin.set_state(led_state).unwrap();
                while button_pin.is_low().unwrap() {}
            }
        }
    }
}
