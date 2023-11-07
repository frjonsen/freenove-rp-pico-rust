#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use embedded_hal::PwmPin;
use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal::pwm::{Channel, Slice, SliceId, Slices, ValidSliceMode, B};
use rp_pico::hal::{clocks, pac, Clock, Sio, Watchdog};

// The minimum PWM value (i.e. LED brightness) we want
const LOW: u16 = 0;

// The maximum PWM value (i.e. LED brightness) we want
const HIGH: u16 = 65535;

fn variant_one<I: SliceId, T: ValidSliceMode<I>>(
    mut delay: Delay,
    channel: &mut Channel<Slice<I, T>, B>,
) -> ! {
    loop {
        for i in LOW..HIGH {
            channel.set_duty(i);
            // delay.delay_ms(5);
            delay.delay_us(100);
        }
        for i in (LOW..HIGH).rev() {
            channel.set_duty(i);
            // delay.delay_ms(5);
            delay.delay_us(100);
        }
    }
}

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
    let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let pwms = Slices::new(pac.PWM, &mut pac.RESETS);

    let mut pwm = pwms.pwm7;
    pwm.set_ph_correct();
    pwm.enable();

    let channel = &mut pwm.channel_b;
    channel.output_to(pins.gpio15);
    variant_one(delay, channel);
}
