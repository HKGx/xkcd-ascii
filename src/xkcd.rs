extern crate ureq;
extern crate image;
extern crate rand;

use image::DynamicImage;

use serde::{Serialize, Deserialize};

use std::str::FromStr;
use std::io::Read;
use self::rand::Rng;

#[derive(Debug, Serialize, Deserialize)]
pub struct Comic {
    news: String,
    link: String,
    img: String,
    num: i32,
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

pub fn get_comic_image(comic: &Comic) -> Result<DynamicImage, String> {
    let resp = ureq::get(comic.img.as_str())
        .timeout_connect(8_000)
        .call();
    if !resp.ok() {
        return Err(String::from("Fetching comic image failed."));
    }
    let mut reader = resp.into_reader();
    let mut bytes = vec![];
    reader.read_to_end(&mut bytes).expect("Something just failed. Eh.");
    return match image::load_from_memory(bytes.as_slice()) {
        Ok(i) => Ok(i),
        Err(e) => Err(format!("{:?}", e)),
    };
}

pub fn get_random_comic() -> Result<Comic, String> {
    let mut rng = rand::thread_rng();
    let latest = get_latest_comic()
        .expect("Failed to fetch latest comic for random comic.");
    let comic_id = rng.gen_range(1, latest.num);
    get_comic(comic_id)
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
    if id < 1 {
        return Err(String::from("Invalid id of the comic. Should be bigger than 0."));
    }
    let resp = ureq::get(format!("https://c.xkcd.com/api-0/jsonp/comic/{}", id).as_str())
        .timeout_connect(4_000)
        .call();

    if !resp.ok() {
        return Err(String::from("Fetching latest comic failed miserably."));
    }

    // this is so retarded, but rustic matches are so lovely
    // i will oseruse them until the end of the world
    match resp.into_string(){
        Ok(s) => match s.as_str() {
            "Need more" => Err(String::from("Provided id was probably too large.")),
            _ => match serde_json::from_str(s.as_str()){
                Ok(c) => Ok(c),
                Err(_) => Err(String::from("Failed to parse latest comic"))
            }
        },
        Err(_) => Err(String::from("Failed to parse latest comic"))

    }
}