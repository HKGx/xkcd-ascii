#[macro_use]
extern crate clap;

use clap::{App, Arg};


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
        .get_matches();

    // TODO: do the thing

    //let id: xkcd::ComicId = value_t!(matches, "id", xkcd::ComicId).unwrap_or(xkcd::ComicId::Random);

    //let comic = xkcd::get_latest_comic().unwrap();

    //let image = xkcd::get_comic_image(comic);

    //ascii_that::print_image(image.unwrap(), Some(80), Some(80));
}

fn is_comic_id(val: String) -> Result<(), String> {
    return match val.parse::<xkcd::ComicId>() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.parse().unwrap())
    };
}