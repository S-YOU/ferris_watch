use log::debug;
use clap::{App, Arg};

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

    Ok(())
}
