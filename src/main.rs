extern crate ansi_term;
use ansi_term::Style;

fn main() {
    println!(
        "{} and this is not", Style::new().bold().paint("This is bold"));
}
