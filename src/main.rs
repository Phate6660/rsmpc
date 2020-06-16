use clap::{Arg, App, SubCommand};
use mpd::{Client, Song, Stats, Status};

fn main() {
    let matches = App::new("rsmpc")
        .version("0.0.1")
        .about("\nmpc, but implemented in Rust.")
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
            .about("Print MPD's status"))
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
    if matches.is_present("restart") {
        c.rewind(0).unwrap();
    } else if matches.is_present("prev") {
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
    } else if matches.is_present("stats") {
        let stats: Stats = c.stats().unwrap();
        let arts = stats.artists;
        let albs = stats.albums;
        let sngs = stats.songs;
        let time = stats.db_playtime.num_seconds();
        let pltm: String = if time > 86400 {
            let t = time / 60 / 60 / 24;
            let pltm = t.to_string() + "d";
            pltm
        } else if time > 3600 {
            let t = time / 60 / 60;
            let pltm = t.to_string() + "h";
            pltm
        } else if time > 60 {
            let t = time / 60;
            let pltm = t.to_string() + "m";
            pltm
        } else {
            println!("Could not calculate DB Playtime.");
            let pltm = "N/A".to_string();
            pltm
        };
        println!("Songs:       {}\nAlbums:      {}\nArtists:     {}\nDB Playtime: {}", sngs, albs, arts, pltm);
    } else if matches.is_present("status") {
        let status: Status = c.status().unwrap();
        let volume = status.volume;
        let repeat = status.repeat;
        let random = status.random;
        let single = status.single;
        let consume = status.consume;
        let state = status.state;
        println!("Volume:  {}%\nRepeat:  {}\nRandom:  {}\nSingle:  {}\nConsume: {}\nState:   {:?}", volume, repeat, random, single, consume, state);
    } else if let Some(matches) = matches.subcommand_matches("set") {
        if matches.is_present("volume") {
            let ivol = matches.value_of("volume").unwrap();
            let vol: i8 = ivol.parse::<i8>().unwrap();
            c.volume(vol).unwrap();
        } else if matches.is_present("repeat") {
            let inrep = matches.value_of("repeat").unwrap();
            let outrep: bool = if inrep == "off" {
                false
            } else if inrep == "on" {
                true
            } else {
                println!("Could not set the repeat mode! Defaulting to off.");
                false
            };
            c.repeat(outrep).unwrap();
        } else if matches.is_present("random") {
            let inran = matches.value_of("random").unwrap();
            let outran: bool = if inran == "off" {
                false
            } else if inran == "on" {
                true
            } else {
                println!("Could not set the random mode! Defaulting to off.");
                false
             };
            c.random(outran).unwrap();
        } else if matches.is_present("single") {
            let insin = matches.value_of("single").unwrap();
            let outsin: bool = if insin == "off" {
                false
            } else if insin == "on" {
                true
            } else {
                println!("Could not set the single mode! Defaulting to off.");
                false
            };
            c.single(outsin).unwrap();
        } else if matches.is_present("consume") {
            let incon = matches.value_of("consume").unwrap();
            let outcon: bool = if incon == "off" {
                false
            } else if incon == "on" {
                true
            } else {
                println!("Could not set the consume mode! Defaulting to off.");
                false
            };
            c.consume(outcon).unwrap();
        }
    }
}
