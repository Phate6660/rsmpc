use clap::{App, SubCommand};
use mpd::Client;

fn main() {
    let matches = App::new("rsmpc")
        .version("0.0.1")
        .about("\nmpc, but implemented in Rust.")
        .subcommand(SubCommand::with_name("prev")
            .about("Play the previous song."))
        .subcommand(SubCommand::with_name("toggle")
            .about("Toggle the player state between play and pause."))
        .subcommand(SubCommand::with_name("next")
            .about("Play the next song."))
        .get_matches();
    let mut c = Client::connect("127.0.0.1:6600").unwrap();
    if matches.is_present("prev") {
        c.prev().unwrap();
    } else if matches.is_present("toggle") {
        c.toggle_pause().unwrap();
    } else if matches.is_present("next") {
        c.next().unwrap();
    }
}
