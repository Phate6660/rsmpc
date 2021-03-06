use mpd::{State, Status, Song};
use sedregex::find_and_replace;
use std::fmt;

// Struct and impl for printing the state as a string
struct PlayState {
    sta: State,
}

impl fmt::Display for PlayState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.sta)
    }
}

fn format_time(time: i64) -> String {
    let minutes = (time / 60) % 60;
    let seconds = time % 60;

    format!("{:0>2}:{:0>2}", minutes, seconds)
}

pub fn obtain_status(song: Song, status: Status) {
    let art = song.tags.get("Artist").unwrap();
    let tit = song.title.as_ref().unwrap();
    let state_pre_string = status.state;
    let state = PlayState { sta: state_pre_string }.to_string();
    let position = song.place.unwrap().pos + 1; // starts counting at 0, add 1 to re-align it with what people expect
    let count = status.queue_len;
    let elap = status.elapsed.unwrap().num_seconds();
    let elapsed = format_time(elap);
    let dur = status.duration.unwrap().num_seconds();
    let duration = format_time(dur);
    let volume = status.volume;
    let repeat = status.repeat;
    let random = status.random;
    let single = status.single;
    let consume = status.consume;
    // artist - title
    let row_one = format!("{} - {}", art, tit);
    let row_two_pre_filter = format!("[{}]   #{}/{}   {}/{}", state, position, count, elapsed, duration);
    // [state] #playlist_current/total elapsed/duration
    let row_two = find_and_replace(&row_two_pre_filter, &["s/Play/Playing/g", "s/Pause/Paused/g"]).unwrap().to_string();
    let row_three_pre_filter = format!("Volume: {}%   Repeat: {}   Random: {}   Single: {}   Consume: {}", volume, repeat, random, single, consume);
    // Volume: percentage  Repeat: on/off  Random: on/off  Single: on/off  Consume: on/off
    let row_three = find_and_replace(&row_three_pre_filter, &["s/false/off/g", "s/true/on/g"]).unwrap().to_string();
    println!("{}\n{}\n{}", row_one, row_two, row_three);
}
