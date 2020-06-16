use mpd::Status;

pub fn obtain_status(status: Status) {
    let volume = status.volume;
    let repeat = status.repeat;
    let random = status.random;
    let single = status.single;
    let consume = status.consume;
    let state = status.state;
    println!("Volume:  {}%\nRepeat:  {}\nRandom:  {}\nSingle:  {}\nConsume: {}\nState:   {:?}", volume, repeat, random, single, consume, state);
}
