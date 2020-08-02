use mpd::Stats;

fn calc_time(time: i64) -> String {
    if time > 86400 {
        let t = time / 60 / 60 / 24;
        return t.to_string() + &" days".to_string()
    } else if time > 3600 {
        let t = time / 60 / 60;
        return t.to_string() + &" hours".to_string()
    } else if time > 60 {
        let t = time / 60;
        return t.to_string() + &" minutes".to_string()
    } else {
        println!("Could not calculate time.");
        return "N/A".to_string()
    };
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
