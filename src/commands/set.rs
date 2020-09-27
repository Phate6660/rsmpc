#![forbid(unsafe_code)]

use clap::ArgMatches;
use mpd::Client;

pub fn consume(matches: &ArgMatches, c: &mut Client) {
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

pub fn random(matches: &ArgMatches, c: &mut Client) {
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
}

pub fn repeat(matches: &ArgMatches, c: &mut Client) {
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
}

pub fn single(matches: &ArgMatches, c: &mut Client) {
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
}

pub fn volume(matches: &ArgMatches, c: &mut Client) {
    let ivol = matches.value_of("volume").unwrap();
    let vol: i8 = ivol.parse::<i8>().unwrap();
    c.volume(vol).unwrap();
}
