#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#[cfg(test)]
use core::iter::Iterator;

use embedded_hal::PwmPin;
#[cfg(not(test))]
use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal::pwm::{FreeRunning, Slice, SliceId, Slices};
use rp_pico::hal::{clocks, pac, Clock, Sio, Watchdog};

const DELAY_TIMES: u32 = 50;

// The minimum PWM value (i.e. LED brightness) we want
const LOW: u16 = 0;

// The maximum PWM value (i.e. LED brightness) we want
const HIGH: u16 = 65535;

const DUTIES: &[u16] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4095, 2047, 1023, 512, 256, 64, 32, 16, 8, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0,
];

fn remap(value: u16, from_low: u16, from_high: u16, to_low: u16, to_high: u16) -> u16 {
    let from_range = from_high - from_low;
    let to_range = to_high - to_low;

    let value_perctentage_of_from = value as f32 / from_range as f32;
    let to_percentage = value_perctentage_of_from * (to_range as f32);

    let rounded = to_percentage + to_low as f32;

    rounded as u16
}

fn initialize_pwm<T: SliceId>(pwm: &mut Slice<T, FreeRunning>) {
    pwm.set_ph_correct();
    pwm.enable();
}

fn initialize_pwms(slices: &mut Slices) {
    initialize_pwm(&mut slices.pwm0);
    initialize_pwm(&mut slices.pwm1);
    initialize_pwm(&mut slices.pwm2);
    initialize_pwm(&mut slices.pwm3);
    initialize_pwm(&mut slices.pwm4);
    initialize_pwm(&mut slices.pwm5);
    initialize_pwm(&mut slices.pwm6);
    initialize_pwm(&mut slices.pwm7);
}

#[cfg_attr(not(test), entry)]
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

    let mut pwms = Slices::new(pac.PWM, &mut pac.RESETS);
    initialize_pwms(&mut pwms);

    pwms.pwm0.channel_a.output_to(pins.gpio16);
    pwms.pwm0.channel_b.output_to(pins.gpio17);
    pwms.pwm1.channel_a.output_to(pins.gpio18);
    pwms.pwm1.channel_b.output_to(pins.gpio19);
    pwms.pwm2.channel_a.output_to(pins.gpio20);
    pwms.pwm2.channel_b.output_to(pins.gpio21);
    pwms.pwm3.channel_a.output_to(pins.gpio22);
    pwms.pwm5.channel_a.output_to(pins.gpio26);
    pwms.pwm5.channel_b.output_to(pins.gpio27);
    pwms.pwm6.channel_a.output_to(pins.gpio28);

    let t: [&mut dyn PwmPin<Duty = u16>; 10] = [
        &mut pwms.pwm0.channel_a,
        &mut pwms.pwm0.channel_b,
        &mut pwms.pwm1.channel_a,
        &mut pwms.pwm1.channel_b,
        &mut pwms.pwm2.channel_a,
        &mut pwms.pwm2.channel_b,
        &mut pwms.pwm3.channel_a,
        &mut pwms.pwm5.channel_a,
        &mut pwms.pwm5.channel_b,
        &mut pwms.pwm6.channel_a,
    ];

    loop {
        for i in 0..20 {
            for j in 0..t.len() {
                let duty = remap(DUTIES[i + j], 0, 4095, LOW, HIGH);
                t[j].set_duty(duty);
            }
            delay.delay_ms(DELAY_TIMES);
        }
        for i in 0..20 {
            for j in 0..t.len() {
                let duty = remap(DUTIES[i + j], 0, 4095, LOW, HIGH);
                t[t.len() - j - 1].set_duty(duty);
            }
            delay.delay_ms(DELAY_TIMES);
        }
    }
}

#[cfg(test)]
mod tests {
    use core::prelude::rust_2021::test;

    use crate::remap;
    #[test]
    fn test_remap() {
        let value = 5;
        let from_min = 0;
        let from_max = 10;
        let to_min = 0;
        let to_max = 100;
        let result = remap(value, from_min, from_max, to_min, to_max);
        assert_eq!(50, result);
    }
}
