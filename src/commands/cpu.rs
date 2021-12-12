use billboard::Billboard;
use clap::{App, ArgMatches};
use sysinfo::SystemExt;

pub fn cpu_info_subcommand() -> App<'static> {
    App::new("pcinfo").about("Show informatoins of your computer")
}

pub fn cpu_info_handler(_args: &ArgMatches) {
    let mut system = sysinfo::System::new_all();
    system.refresh_all();

    let none_str = String::from("none");
    let arch = system.name().unwrap_or(none_str.clone());
    let platform = system.os_version().unwrap_or(none_str.clone());
    let version = system.os_version().unwrap_or(none_str.clone());

    let txt: String = format!(
        "ARCH: {}\nPlatform: {}\nVersion: {}\n",
        arch, platform, version,
    );
    Billboard::default().display(txt.as_str())
}
