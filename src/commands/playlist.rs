#![forbid(unsafe_code)]

use mpd::Client;

pub fn playlist(c: &mut Client) {
    let queue = c.queue().unwrap();
    for x in queue {
        println!("{} - {}", x.tags.get("Artist").unwrap(), x.title.unwrap());
    }
}
