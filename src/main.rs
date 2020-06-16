use clap::{Arg, App, SubCommand};
use mpd::{Client, Song, Stats, Status};

mod commands;
use commands::{current::current, playlist::playlist,
               set::{consume, random, repeat, single, volume},
               stats::obtain_stats, status::obtain_status};

fn main() {
    let matches = App::new("rsmpc")
        .version("0.0.1")
        .about("\nmpc, but implemented in Rust.")
        .subcommand(SubCommand::with_name("playlist")
            .about("Prints all songs in the current queue."))
        .subcommand(SubCommand::with_name("restart")
            .about("Restarts the current song from the beginning."))
        .subcommand(SubCommand::with_name("prev")
            .about("Play the previous song."))
        .subcommand(SubCommand::with_name("toggle")
            .about("Toggle the player state between play and pause."))
        .subcommand(SubCommand::with_name("next")
            .about("Play the next song."))
        .subcommand(SubCommand::with_name("current")
            .about("Print the current song in \"artist - title\" format."))
        .subcommand(SubCommand::with_name("stats")
            .about("Print MPD stats."))
        .subcommand(SubCommand::with_name("status")
            .about("Print MPD's status."))
        .subcommand(SubCommand::with_name("set")
            .about("Set different options for MPD.")
            .arg(Arg::with_name("volume")
                .long("volume")
                .help("Set the volume.")
                .value_name("PERCENTAGE")
                .takes_value(true))
            .arg(Arg::with_name("repeat")
                .long("repeat")
                .help("Disable or enable repeat mode.")
                .value_name("off/on")
                .takes_value(true))
            .arg(Arg::with_name("random")
                .long("random")
                .help("Disable or enable random mode.")
                .value_name("off/on")
                .takes_value(true))
            .arg(Arg::with_name("single")
                .long("single")
                .help("Disable or enable single mode.")
                .value_name("off/on")
                .takes_value(true))
            .arg(Arg::with_name("consume")
                .long("consume")
                .help("Disable or enable consume mode.")
                .value_name("off/on")
                .takes_value(true)))
        .get_matches();
    let mut c = Client::connect("127.0.0.1:6600").unwrap();
    let song: Song = c.currentsong().unwrap().unwrap();
    let stats: Stats = c.stats().unwrap();
    let status: Status = c.status().unwrap();
    if matches.is_present("playlist") {
        playlist(&mut c);
    } else if matches.is_present("restart") {
        c.rewind(0).unwrap();
    } else if matches.is_present("prev") {
        c.prev().unwrap();
    } else if matches.is_present("toggle") {
        c.toggle_pause().unwrap();
    } else if matches.is_present("next") {
        c.next().unwrap();
    } else if matches.is_present("current") {
        current(song);
    } else if matches.is_present("stats") {
        obtain_stats(stats);
    } else if matches.is_present("status") {
        obtain_status(status);
    } else if let Some(matches) = matches.subcommand_matches("set") {
        if matches.is_present("volume") {
            volume(matches, &mut c);
        } else if matches.is_present("repeat") {
            repeat(matches, &mut c);
        } else if matches.is_present("random") {
            random(matches, &mut c);
        } else if matches.is_present("single") {
            single(matches, &mut c);
        } else if matches.is_present("consume") {
            consume(matches, &mut c);
        }
    }
}
