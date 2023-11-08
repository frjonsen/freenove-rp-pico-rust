#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use embedded_hal::PwmPin;
#[cfg(not(test))]
use panic_halt as _;
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

fn remap(value: u16, from_low: u16, from_high: u16, to_low: u16, to_high: u16) -> u16 {
    let from_range = from_high - from_low;
    let to_range = to_high - to_low;

    let value_perctentage_of_from = value as f32 / from_range as f32;
    let to_percentage = value_perctentage_of_from * (to_range as f32);

    let rounded = to_percentage + to_low as f32;

    rounded as u16
}

fn wheel(pos: u16) -> u32 {
    let mut wheel_pos: u32 = pos as u32 % 0xff;
    if wheel_pos < 85 {
        let result = ((255 - wheel_pos * 3) << 16) | ((wheel_pos * 3) << 8);
        return result;
    }
    if wheel_pos < 170 {
        wheel_pos -= 85;
        let result = ((255 - wheel_pos * 3) << 8) | (wheel_pos * 3);
        return result;
    }

    wheel_pos -= 170;

    (wheel_pos * 3) << 16 | (255 - wheel_pos * 3)
}

fn remap_colors(color: u16) -> u16 {
    remap(color, 0, 255, LOW, HIGH)
}

struct Colors {
    red: u16,
    green: u16,
    blue: u16,
}

fn get_colors(i: u16, colors: &mut Colors) {
    let new_color = wheel(i);
    let red: u16 = ((new_color >> 16) & 0xFF).try_into().unwrap();
    let green: u16 = ((new_color >> 8) & 0xFF).try_into().unwrap();
    let blue: u16 = (new_color & 0xFF).try_into().unwrap();

    colors.blue = remap_colors(blue);
    colors.green = remap_colors(green);
    colors.red = remap_colors(red);
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

    let pwms = Slices::new(pac.PWM, &mut pac.RESETS);

    let pwm5 = initialize_pwm(pwms.pwm5);
    let pwm6 = initialize_pwm(pwms.pwm6);

    let mut pin11 = pwm5.channel_b;
    pin11.output_to(pins.gpio11);
    let mut pin12 = pwm6.channel_a;
    pin12.output_to(pins.gpio12);
    let mut pin13 = pwm6.channel_b;

    let mut current_colors = Colors {
        blue: 0,
        green: 0,
        red: 0,
    };
    loop {
        for i in 0..256 {
            get_colors(i, &mut current_colors);
            pin13.set_duty(HIGH - current_colors.red);
            pin12.set_duty(HIGH - current_colors.green);
            pin11.set_duty(HIGH - current_colors.blue);
            delay.delay_ms(100);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_colors, wheel, Colors};

    #[test]
    fn test_wheel() {
        let mut colors = Colors {
            red: 0,
            green: 0,
            blue: 0,
        };
        for i in 0..256 {
            get_colors(i, &mut colors);
        }
    }
}
