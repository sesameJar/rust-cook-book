extern crate ansi_term;
use ansi_term::{Color, Style};

fn main() {
    println!("{},{} and {}",
    Color::Yellow.paint("This is Colored"),
    Style::new().bold().paint("This is only Bold"),
    Color::Yellow.bold().paint("This is bold and colored"));
}
