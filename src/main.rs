use log::debug;
use clap::App;

fn main() -> Result<(), failure::Error> {
    env_logger::init();
    debug!("ferris_watch starting...");

    let matches = App::new("ferris_watch") .version("0.1.0")
        .author("Your Name <yourname@example.com>") .about("cute watch command") .get_matches();

    Ok(())
}
