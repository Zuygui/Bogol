#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg};

mod commands;

fn main() {
    let matches = App::new(crate_name!())
        .about("🚀 The best CLI for everyday life for everyone.")
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author(crate_authors!())
        .subcommand(commands::cpu::cpu_info_subcommand())
        .get_matches();

    match matches.subcommand() {
        Some(("pcinfo", pcinfo_matches)) => {
            commands::cpu::cpu_info_handler(pcinfo_matches);
        }
        _ => unimplemented!(),
    }
}
