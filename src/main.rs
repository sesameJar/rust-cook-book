extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("My test Program")
        .version("0.0.1")
        .author("Mehrad Kavian")
        .about("Some stupid command line App")
        .arg(
            Arg::with_name("multiply")
                .short("m")
                .long("mul")
                .takes_value(true)
                .help("received two number and multiplies them"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("num")
                .takes_value(true)
                .help("your fucking favorite number you mother bitch"),
        )
        .get_matches();

    let numbers = matches.value_of("multiply");
    match numbers {
        None => println!("You are supposed to provide a number you stupid monkey!"),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => println!("{}", n),
                Err(_) => println!("NOT A NUMBER STUPID FUCKING HORSE"),
            }
        }
    }
}
