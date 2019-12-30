extern crate ureq;
extern crate image;

use image::DynamicImage;

use serde::{Serialize, Deserialize};

use std::str::FromStr;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub struct Comic {
    news: String,
    link: String,
    img: String,
    num: i64,
    day: String,
    year: String,
    month: String,
    title: String,
    safe_title: String,
    transcript: String,
    alt: String,
}


#[derive(Debug)]
pub enum ComicId {
    Random,
    Number(i32),
}

impl FromStr for ComicId {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim() == "" {
            return Ok(ComicId::Random);
        }
        let num: i32 = match s.trim().parse() {
            Ok(n) => { n }
            Err(_) => { return Err("id must be a integer"); }
        };
        return if num < 1
        { Err("id must be bigger than 0") } else { Ok(ComicId::Number(num)) };
    }
}

pub fn get_comic_image(comic: Comic) -> Result<DynamicImage, String> {
    let resp = ureq::get(comic.img.as_str())
        .timeout_connect(8_000)
        .call();
    if !resp.ok() {
        return Err(String::from("Fetching comic image failed."));
    }
    let mut reader = resp.into_reader();
    let mut bytes = vec![];
    reader.read_to_end(&mut bytes).expect("Something just failed. Eh.");
    return match image::load_from_memory_with_format(bytes.as_slice(), image::ImageFormat::PNG) {
        Ok(i) => Ok(i),
        Err(e) => Err(format!("{:?}", e)),
    };
}

pub fn get_latest_comic() -> Result<Comic, String> {
    let resp = ureq::get("https://c.xkcd.com/api-0/jsonp/comic")
        .timeout_connect(4_000)
        .call();
    if !resp.ok() {
        return Err(String::from("Fetching latest comic failed miserably."));
    }
    match serde_json::from_str(resp.into_string().unwrap().as_str()) {
        Ok(c) => Ok(c),
        Err(_) => Err(String::from("Failed to parse latest comic"))
    }
}

pub fn get_comic(id: i32) -> Result<Comic, String> {
    // TODO: add the thing
    unimplemented!();
}