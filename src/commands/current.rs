#![forbid(unsafe_code)]

use mpd::Song;

pub fn current(song: Song) {
    let tit = song.title.as_ref().unwrap();
    let art = song.tags.get("Artist").unwrap();
    println!("{} - {}", art, tit);
}
