pub use crate::console::{
    clear,
    read_char,
    read_input,
    read_pass,
    random_color,
    Console,
    StdConsole,
}; 

#[cfg(test)]
mod tests {
    use termcolor::Color;

    use super::*;

    #[test]
    fn random_color_returns_rgb() {
        let c = random_color();
        match c {
            Color::Rgb(r, g, b) => {
                // Values are u8 so bounds-checking against 0..=255 is unnecessary;
                // we just ensure the variant contains three components.
                let _ = r;
                let _ = g;
                let _ = b;
            }
            _ => panic!("expected Rgb color"),
        }
    }
}
