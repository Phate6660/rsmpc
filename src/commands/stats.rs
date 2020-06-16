use mpd::Stats;

pub fn obtain_stats(stats: Stats) {
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
}
