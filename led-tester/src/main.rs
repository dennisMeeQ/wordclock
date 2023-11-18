// rust command line tool to test led strip
// arguements:
// -n: number of leds
// -m: led to turn on
// -c: color to turn on

use core::time;
use rand::Rng;

use clap::Parser;
use ws2818_rgb_led_spi_driver::adapter_gen::WS28xxAdapter;
use ws2818_rgb_led_spi_driver::adapter_spi::WS28xxSpiAdapter;
use ws2818_rgb_led_spi_driver::encoding::encode_rgb;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of LEDs on the strip
    #[arg(short, long)]
    num: usize,

    /// The number of the LED to turn on
    #[arg(short, long)]
    turn_on: usize,

    /// Mode to run
    #[clap(value_enum)]
    mode: Mode,

    /// Color to use
    #[clap(value_enum)]
    color: Color,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Mode {
    Single,
    Animation,
    AllOff,
    Blink,
}

#[derive(clap::ValueEnum, Clone, Debug, Copy)]
enum Color {
    Red,
    Green,
    Blue,
    White,
}

fn main() {
    let args = Args::parse();

    let mut adapter = WS28xxSpiAdapter::new("/dev/spidev0.0").unwrap();

    println!("LED strip with {} LEDs", args.num);

    match args.mode {
        Mode::Single => single_led(&mut adapter, args),
        Mode::Animation => animation(&mut adapter, args),
        Mode::AllOff => all_off(&mut adapter, args),
        Mode::Blink => blink(&mut adapter, args),
    }
}

fn single_led(adapter: &mut WS28xxSpiAdapter, args: Args) {
    let rgb_values = set_color_on_led(args.num, args.turn_on, args.color);
    adapter.write_rgb(&rgb_values).unwrap();
}

fn all_off(adapter: &mut WS28xxSpiAdapter, args: Args) {
    let rgb_values = vec![(0, 0, 0); args.num];
    adapter.write_rgb(&rgb_values).unwrap();
}

fn blink(adapter: &mut WS28xxSpiAdapter, args: Args) {
    let mut counter = 0;
    while counter < 5 {
        let rgb_values = set_color_on_all_leds(args.num, args.color);
        adapter.write_rgb(&rgb_values).unwrap();

        let half_a_second = time::Duration::from_millis(500);
        std::thread::sleep(half_a_second);

        let rgb_values = vec![(0, 0, 0); args.num];
        adapter.write_rgb(&rgb_values).unwrap();
        std::thread::sleep(half_a_second);

        counter += 1;
    }
}

fn animation(adapter: &mut WS28xxSpiAdapter, args: Args) {
    let mut counter = 0;
    while counter < args.num * 2 {
        let current_led = counter % args.num;

        let rgb_values = set_color_on_led(args.num, current_led, random_color());
        adapter.write_rgb(&rgb_values).unwrap();

        let pause = time::Duration::from_millis(300);
        std::thread::sleep(pause);

        counter += 1;
    }
}

fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    let num: u8 = rng.gen_range(0..4);

    match num {
        0 => Color::Red,
        1 => Color::Green,
        2 => Color::Blue,
        3 => Color::White,
        _ => Color::White,
    }
}

fn set_color_on_all_leds(num: usize, color: Color) -> Vec<(u8, u8, u8)> {
    let mut strip: Vec<(u8, u8, u8)>;

    match color {
        Color::Red => strip = vec![(255, 0, 0); num],
        Color::Green => strip = vec![(0, 255, 0); num],
        Color::Blue => strip = vec![(0, 0, 255); num],
        Color::White => strip = vec![(255, 255, 255); num],
    }

    strip
}

fn set_color_on_led(num: usize, led: usize, color: Color) -> Vec<(u8, u8, u8)> {
    let mut strip: Vec<(u8, u8, u8)> = vec![(0, 0, 0); num];

    match color {
        Color::Red => strip[led] = (255, 0, 0),
        Color::Green => strip[led] = (0, 255, 0),
        Color::Blue => strip[led] = (0, 0, 255),
        Color::White => strip[led] = (255, 255, 255),
    }

    strip
}
