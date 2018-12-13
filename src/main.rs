use log::debug;
use clap::{App, Arg, value_t};
use std::process::Command;

fn main() -> Result<(), failure::Error> {
    env_logger::init();
    debug!("ferris_watch starting...");

    let matches = App::new("ferris_watch").version("0.1.0")
        .author("Your Name <yourname@example.com>").about("cute watch command")
        .arg(
            Arg::with_name("command")
                .required(true)
                .multiple(true)
                .help("The command to run periodically"),
        )
        .arg(
            Arg::with_name("interval")
                .long("interval")
                .short("n")
                .takes_value(true) .default_value("2.0")
                .help("The period to run a command"),
        )
        .get_matches();

    let command =
        matches.values_of("command").unwrap().collect::<Vec<_>>(); let interval = value_t!(matches, "interval", f64)?;
    debug!("command = {:?}", command);
    debug!("interval = {:?}", interval);

    // RUST_LOG=ferris_watch=debug cargo run -- -n 0.5 -- ls -a
//    2018-12-13T11:27:45Z DEBUG ferris_watch] ferris_watch starting...
//        [2018-12-13T11:27:45Z DEBUG ferris_watch] command = ["ls", "-a"]
//    [2018-12-13T11:27:45Z DEBUG ferris_watch] interval = 0.5

    let output = Command::new(command[0]).args(&command[1..]).output()?; debug!("output = {:?}", output);
    let output = String::from_utf8_lossy(&output.stdout);
    println!("{}", output);
    // 20202032n [master] ~/repo/rust/handson4/ferris_watch/ %
    /*
.
..
.git
.gitignore
.idea
Cargo.lock
Cargo.toml
src
target
    */

    Ok(())
}
