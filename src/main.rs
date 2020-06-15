use clap::{App, SubCommand};
use mpd::{Client, Song};

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
        .subcommand(SubCommand::with_name("current")
            .about("Print the current song in \"artist - title\" format."))
        .get_matches();
    let mut c = Client::connect("127.0.0.1:6600").unwrap();
    if matches.is_present("prev") {
        c.prev().unwrap();
    } else if matches.is_present("toggle") {
        c.toggle_pause().unwrap();
    } else if matches.is_present("next") {
        c.next().unwrap();
    } else if matches.is_present("current") {
        let song: Song = c.currentsong().unwrap().unwrap();
        let tit = song.title.as_ref().unwrap();
        let art = song.tags.get("Artist").unwrap();
        println!("{} - {}", art, tit);
    }
}
