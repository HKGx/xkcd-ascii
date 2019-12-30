#[macro_use]
extern crate clap;

use std::str::FromStr;
use clap::{App, Arg};

#[derive(Debug)]
enum ComicId {
    Random,
    Number(i32),
}

impl FromStr for ComicId {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim() == ""{
            return Ok(ComicId::Random);
        }
        let num: i32 = match s.trim().parse() {
            Ok(n) => {n},
            Err(_) => {return Err("id must be a integer")}
        };
        return if num < 1
        { Err("id must be bigger than 0")}
        else { Ok(ComicId::Number(num))};
    }


}

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

    let id: ComicId = value_t!(matches, "id", ComicId).unwrap_or(ComicId::Random);

    println!("ASCII Generator not yet implemented. 😦\nThe id you've passed is: {:?}", id);
}

fn is_comic_id(val: String) -> Result<(), String> {
    return match val.parse::<ComicId>() {
        Ok(_) => Ok(()),
        Err(e) =>  Err(e.parse().unwrap())
    };
}