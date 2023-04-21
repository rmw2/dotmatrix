#![no_std]
#![no_main]

use panic_halt as _;

use seeeduino_xiao_rp2040 as bsp;

use bsp::{entry, hal, Pins};
use hal::{gpio, pac, Clock};
use gpio::pin::{Pin, PushPullOutput};

use cortex_m::delay::Delay;
use embedded_hal::digital::v2::OutputPin;
use max7219::MAX7219;

mod font;
use font::CP437FONT;

const N_DISPLAYS: usize = 4;
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[entry]
fn main() -> ! {
    // Access to pac & core objects via singletons, hence the shadowing.
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed only by the clock setup code.
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).ok().unwrap();

    // Delay for timing control uses system clock.
    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Access to GPIO pins is through singleton struct.
    let sio = hal::Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Indicator LED for blinking comms during execution.
    let mut led = pins.led_red.into_push_pull_output();

    // SPI pins for Seeed Xiao RP2040.  See pinout & BSP for details.
    let sck = pins.sck.into_push_pull_output();
    let data = pins.mosi.into_push_pull_output();
    let cs = pins.rx.into_push_pull_output();

    let mut display = MAX7219::from_pins(N_DISPLAYS, data, cs, sck).unwrap();
    display.power_on().unwrap();

    // Setup is done!
    fastblink(&mut led, &mut delay, 10, 100);

    loop {
        display.clear_display(0).unwrap();
        display.write_raw(3, &px_right(CP437FONT[b'W' as usize])).unwrap();
        display.write_raw(2, &px_down(CP437FONT[b'E' as usize])).unwrap();
        display.write_raw(1, &CP437FONT[b'E' as usize]).unwrap();
        display.write_raw(0, &px_down(CP437FONT[b'D' as usize])).unwrap();
        delay.delay_ms(200);

        display.clear_display(0).unwrap();
        display.write_raw(3, &CP437FONT[b'W' as usize]).unwrap();
        display.write_raw(2, &CP437FONT[b'E' as usize]).unwrap();
        display.write_raw(1, &px_right(px_down(CP437FONT[b'E' as usize]))).unwrap();
        display.write_raw(0, &CP437FONT[b'D' as usize]).unwrap();
        delay.delay_ms(200);

        fastblink(&mut led, &mut delay, 2, 10);
    }
}

fn px_down(letter: [u8; 8]) -> [u8; 8] {
    let mut dest = [0u8; 8];
    dest[..7].copy_from_slice(&letter[1..]);
    dest
}

fn px_right(letter: [u8; 8]) -> [u8; 8] {
    letter.map(|b| b << 1 & 0b11111110)
}

// Takes a delay and LED pin & flashes it `n` times, on and off for `ms` each.
fn fastblink(
        // Typing is weird.  Gpio17 is the red LED for Seeed Xiao.
        led: &mut Pin<gpio::bank0::Gpio17, PushPullOutput>,
        delay: &mut Delay, n: u8, ms: u32) {
    for _ in 0..n {
        led.set_high().unwrap();
        delay.delay_ms(ms);
        led.set_low().unwrap();
        delay.delay_ms(ms);
    }
    delay.delay_ms(1000);
}
