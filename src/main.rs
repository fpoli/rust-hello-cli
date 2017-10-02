#[macro_use]
extern crate clap;
extern crate lib;
use clap::{Arg, App, AppSettings};
use lib::build_message;

fn main() {
    let matches = App::new("hello")
        .version(crate_version!())
        .usage("hello [OPTIONS]")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(Arg::with_name("name")
            .long("name")
            .short("n")
            .takes_value(true)
            .help("Use name (default: World)"))
            .get_matches();

    let opt_name = matches.value_of("name");
    let message = build_message(opt_name);
    println!("{}", message);
}
