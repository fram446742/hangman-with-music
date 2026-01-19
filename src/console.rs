use rand::Rng;
use rpassword::read_password;
use std::io::{self, Write};
use termcolor::Color;

/// Lee una línea desde stdin y devuelve la cadena sin el salto de línea final.
///
/// En caso de error devuelve una cadena vacía y escribe un mensaje de error
/// en stderr.
pub fn read_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim_end().to_string(),
        Err(e) => {
            eprintln!("Failed to read input: {}", e);
            String::new()
        }
    }
}

/// Lee el primer carácter disponible desde stdin (después de hacer trim).
pub fn read_char() -> Option<char> {
    read_input().chars().next()
}

/// Lee una contraseña (entrada no eco) desde stdin.
pub fn read_pass() -> String {
    match read_password() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to read password: {}", e);
            String::new()
        }
    }
}

/// Limpia la pantalla usando códigos ANSI y maneja posibles errores de flush.
pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
    if let Err(e) = io::stdout().flush() {
        eprintln!("Failed to flush stdout: {}", e);
    }
}

fn hsl_to_termcolor(h: f64, s: f64, l: f64) -> Color {
    // Normalizar hue en [0, 360)
    let h = h.rem_euclid(360.0);

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0).rem_euclid(2.0) - 1.0).abs());
    let m = l - c / 2.0;

    let (r_f, g_f, b_f) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let r = ((r_f + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    let g = ((g_f + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    let b = ((b_f + m) * 255.0).round().clamp(0.0, 255.0) as u8;

    Color::Rgb(r, g, b)
}

/// Genera un color aleatorio en espacio HSL y lo convierte a RGB para
/// su uso con `termcolor::Color::Rgb`.
pub fn random_color() -> Color {
    let mut rng = rand::rng();
    let hue = rng.random_range(0.0..360.0);
    let saturation = rng.random_range(0.5..1.0);
    let lightness = rng.random_range(0.5..0.9);

    hsl_to_termcolor(hue, saturation, lightness)
}

/// Small trait to allow injecting/testing console behavior.
pub trait Console {
    fn read_input(&self) -> String {
        read_input()
    }
    fn read_char(&self) -> Option<char> {
        read_char()
    }
    fn read_pass(&self) -> String {
        read_pass()
    }
    fn clear(&self) {
        clear()
    }
    fn random_color(&self) -> Color {
        random_color()
    }
}

/// Default standard console implementation.
pub struct StdConsole;
impl Console for StdConsole {}

#[cfg(test)]
mod tests {
    use super::*;
    use termcolor::Color;

    #[test]
    fn random_color_returns_rgb() {
        let c = random_color();
        match c {
            Color::Rgb(_r, _g, _b) => {}
            _ => panic!("expected Rgb color"),
        }
    }
}
