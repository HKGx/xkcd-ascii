#[macro_use]
extern crate clap;

use clap::{App, Arg};
use crate::xkcd::Comic;

mod ascii_that;
mod xkcd;


fn main() {
    let matches = App::new("xkcd ascii generator")
        .version("0.1")
        .author("HKG")
        .about("Converts xkcd comics to ascii")
        .arg(Arg::with_name("id")
            .help("Id of xkcd comic. If not provided then a random comic is fetched.")
            .allow_hyphen_values(true)
            .required(false)
            .index(1)
            .validator(is_comic_id))
        .arg(Arg::with_name("debug")
            .hidden(true)
            .long("debug"))
        .get_matches();

    // TODO: do the thing

    let id: xkcd::ComicId = value_t!(matches, "id", xkcd::ComicId).unwrap_or(xkcd::ComicId::Random);
    let debug = matches.is_present("debug");

    let comic: Comic = match id {
        xkcd::ComicId::Random => xkcd::get_random_comic().unwrap(),
        xkcd::ComicId::Number(i) => xkcd::get_comic(i).unwrap(),
    };

    if debug {
        println!("The comic we fetched: {:?}\n", comic)
    }

    let image = xkcd::get_comic_image(&comic).expect("Failed to get comic.");

    // static width, too lazy to implement width and height arguments. maybe later
    ascii_that::print_image(image, Some(80), Some(80));
}

fn is_comic_id(val: String) -> Result<(), String> {
    return match val.parse::<xkcd::ComicId>() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.parse().unwrap())
    };
}