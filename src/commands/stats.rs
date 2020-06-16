use mpd::Stats;

pub fn obtain_stats(stats: Stats) {
    let arts = stats.artists;
    let albs = stats.albums;
    let sngs = stats.songs;
    let time = stats.db_playtime.num_seconds();
    let pltm: String = if time > 86400 {
        let t = time / 60 / 60 / 24;
        t.to_string() + "d"
    } else if time > 3600 {
        let t = time / 60 / 60;
        t.to_string() + "h"
    } else if time > 60 {
        let t = time / 60;
        t.to_string() + "m"
    } else {
        println!("Could not calculate DB Playtime.");
        "N/A".to_string()
    };
    println!("Songs:       {}\nAlbums:      {}\nArtists:     {}\nDB Playtime: {}", sngs, albs, arts, pltm);
}
