use std::process;

#[macro_use]
extern crate clap;
use colored::*;

mod rusage;
mod tty;

use rusage::{Errno, Rusage};

fn main() {
    let app = app_from_crate!()
        .setting(clap::AppSettings::TrailingVarArg)
        .author("")
        .arg(
            clap::Arg::with_name("nocolor")
                .long("no-color")
                .help("Do not use colors in output"),
        )
        .arg(clap::Arg::with_name("command").multiple(true).help(
            r#"Run command and print its rusage structure contents.
If omitted the rusage for the getrusage command itself
is printed."#,
        ));
    let matches = app.get_matches();

    let use_color = !matches.is_present("nocolor");
    let result = matches.values_of_os("command").map_or_else(
        || Ok(Box::new(rusage::Rusage::current) as Box<Fn() -> Result<Rusage, Errno>>),
        |mut args| {
            args.next()
                .ok_or_else(|| {
                    eprintln!("Expected a command but didn't find one");
                    -1
                })
                .and_then(|cmd| {
                    process::Command::new(&cmd)
                        .args(args)
                        .spawn()
                        .and_then(|mut handle| handle.wait())
                        .map_err(|err| {
                            eprintln!("{}: {}", cmd.to_string_lossy(), err);
                            err.raw_os_error().unwrap_or_else(|| -1)
                        })
                        .map(|_| {
                            Box::new(rusage::Rusage::children) as Box<Fn() -> Result<Rusage, Errno>>
                        })
                })
        },
    );

    let status = result
        .and_then(|usage_fn| usage_fn())
        .map(|usage| {
            println!("\n---------------------------\n");
            usage.data().iter().for_each(|(name, value)| {
                let label = if tty::in_tty() && use_color {
                    name.bold()
                } else {
                    name.normal()
                };

                println!("{}: {}", label, value);
            });
            0
        })
        .unwrap_or_else(|err_code| err_code);

    process::exit(status);
}
