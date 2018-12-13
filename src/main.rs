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
        .get_matches();

    Ok(())
}
