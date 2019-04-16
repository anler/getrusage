use std::io;
use std::process;

#[macro_use]
extern crate clap;
use colorful::Colorful;

mod rusage;
mod tty;

fn main() -> io::Result<()> {
    let app = app_from_crate!().author("").arg(
        clap::Arg::with_name("command")
            .help("Run command and print its rusage structure contents."),
    );
    let matches = app.get_matches();
    let value = matches
        .value_of_os("command")
        .expect("expected command argument");
    let result = process::Command::new(value.to_os_string()).spawn();

    result.and_then(|mut handle| handle.wait()).map(|_| {
        println!("\n---------------------------\n");
        rusage::Rusage::children()
            .data()
            .iter()
            .for_each(|(name, value)| {
                let label = if tty::in_tty() {
                    name.bold()
                } else {
                    name.dim()
                };

                println!("{}: {}", label, value);
            });
    })
}
