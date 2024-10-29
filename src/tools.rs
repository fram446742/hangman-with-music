use rand::Rng;
use rpassword::read_password;
use std::io::{ self, Write };
use termcolor::Color;

pub fn read_input() -> String {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("Failed to read input");
    }
    input
}

pub fn read_char() -> Option<char> {
    read_input().trim().chars().next()
}

pub fn read_pass() -> String {
    read_password().unwrap_or_else(|_| {
        eprintln!("Failed to read password");
        String::new()
    })
}

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn hsl_to_termcolor(h: f64, s: f64, l: f64) -> Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - (((h / 60.0) % 2.0) - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match h {
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    let r = ((r + m) * 255.0).round() as u8;
    let g = ((g + m) * 255.0).round() as u8;
    let b = ((b + m) * 255.0).round() as u8;

    Color::Rgb(r, g, b)
}

pub fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    let hue = rng.gen_range(0.0..360.0);
    let saturation = rng.gen_range(0.5..1.0);
    let lightness = rng.gen_range(0.5..0.9);

    hsl_to_termcolor(hue, saturation, lightness)
}
