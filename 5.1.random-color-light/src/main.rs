#![no_std]
#![no_main]

use embedded_hal::PwmPin;
use panic_halt as _;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rp_pico::entry;
use rp_pico::hal::pwm::{FreeRunning, Slice, SliceId, Slices};
use rp_pico::hal::{clocks, pac, Clock, Sio, Watchdog};

// The minimum PWM value (i.e. LED brightness) we want
const LOW: u16 = 0;

// The maximum PWM value (i.e. LED brightness) we want
const HIGH: u16 = 65532;

fn initialize_pwm<T: SliceId>(mut pwm: Slice<T, FreeRunning>) -> Slice<T, FreeRunning> {
    pwm.set_ph_correct();
    pwm.enable();
    pwm
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
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let pwms = Slices::new(pac.PWM, &mut pac.RESETS);

    let pwm5 = initialize_pwm(pwms.pwm5);
    let pwm6 = initialize_pwm(pwms.pwm6);

    let mut pin11 = pwm5.channel_b;
    pin11.output_to(pins.gpio11);
    let mut pin12 = pwm6.channel_a;
    pin12.output_to(pins.gpio12);
    let mut pin13 = pwm6.channel_b;
    pin13.output_to(pins.gpio13);

    let mut rng = SmallRng::seed_from_u64(100);
    // Lower the brightness or it's annoying to look at
    const MAX: u16 = HIGH / 4;
    loop {
        delay.delay_ms(400);
        pin13.set_duty(HIGH - rng.gen_range(LOW..MAX));
        pin12.set_duty(HIGH - rng.gen_range(LOW..MAX));
        pin11.set_duty(HIGH - rng.gen_range(LOW..MAX));
    }
}
