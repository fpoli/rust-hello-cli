use clap::{App, Arg, AppSettings};

pub fn build_cli() -> App<'static, 'static> {
    return App::new("hello")
        .version(crate_version!())
        .usage("hello [OPTIONS]")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(Arg::with_name("name")
            .long("name")
            .short("n")
            .takes_value(true)
            .help("Use name (default: World)"));
}
