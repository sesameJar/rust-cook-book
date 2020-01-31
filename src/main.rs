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
                .number_of_values(2)
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

    let numbers :Vec<&str> = matches.values_of("multiply").unwrap().collect();
    println!("{:?}", numbers);
}
