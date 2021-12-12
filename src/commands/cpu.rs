use billboard::Billboard;
use clap::{App, ArgMatches};

pub fn cpu_info_subcommand() -> App<'static> {
    App::new("pcinfo").about("Show informatoins of your computer")
}

pub fn cpu_info_handler(_args: &ArgMatches) {
    let txt: String = format!("ARCH: {}", "lol");
    Billboard::default().display(txt.as_str())
}
