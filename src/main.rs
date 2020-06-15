use clap::{App, SubCommand};
use mpd::{Client, Song, Stats, Status};

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
        .subcommand(SubCommand::with_name("stats")
            .about("Print MPD stats."))
        .subcommand(SubCommand::with_name("status")
            .about("Print MPD status."))
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
        println!("Songs: {}\nAlbums: {}\nArtists: {}\nDB Playtime: {}", sngs, albs, arts, pltm);
    } else if matches.is_present("status") {
        let status: Status = c.status().unwrap();
        let volume = status.volume;
        let repeat = status.repeat;
        let random = status.random;
        let single = status.single;
        let consume = status.consume;
        let state = status.state;
        println!("Volume:  {}%\nRepeat:  {}\nRandom:  {}\nSingle:  {}\nConsume: {}\nState:   {:?}", volume, repeat, random, single, consume, state);
    }
}
