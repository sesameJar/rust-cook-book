extern crate ansi_term;
use ansi_term::Color;

fn main() {
    println!("This is {} in color, {} in color, {} in color",
    Color::Red.blink().paint("RED"),
    Color::Green.bold().underline().italic().paint("GREEN"),
    Color::Blue.bold().paint("BLUE"));
}