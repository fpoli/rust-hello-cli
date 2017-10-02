#[macro_use]
extern crate clap;

mod lib;
mod cli;

fn main() {
    let matches = cli::build_cli().get_matches();
    let opt_name = matches.value_of("name");
    let message = lib::build_message(opt_name);
    println!("{}", message);
}
