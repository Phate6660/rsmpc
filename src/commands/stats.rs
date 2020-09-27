#![forbid(unsafe_code)]

use mpd::Stats;

fn calc_time(time: i64) -> String {
    let days = if time > 86400 {
        let days_pre = time / 60 / 60 / 24;
        days_pre.to_string() + "d"
    } else {
        "".to_string()
    };
    let hours = if time > 3600 {
        let hours_pre = (time / 60 / 60) % 24;
        hours_pre.to_string() + "h"
    } else {
        "".to_string()
    };
    let minutes = if time > 60 {
        let minutes_pre = (time / 60) % 60;
        minutes_pre.to_string() + "m"
    } else {
        "".to_string()
    };
    format!("{} {} {}", days, hours, minutes).trim().to_string()
}

pub fn obtain_stats(stats: Stats) {
    let arts = stats.artists;
    let albs = stats.albums;
    let sngs = stats.songs;
    let upti = calc_time(stats.uptime.num_seconds());
    let dbti = calc_time(stats.db_playtime.num_seconds());
    // Songs:       amount
    // Albums:      amount
    // Artists:     amount
    // Uptime:      time
    // TODO: Add date/time of the DB updating previously
    // DB Playtime: time
    println!("Songs:       {}\nAlbums:      {}\nArtists:     {}\nUptime:      {}\nDB Playtime: {}", sngs, albs, arts, upti, dbti);
}
