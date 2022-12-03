#![feature(test)]

use chrono::{Datelike, Local, NaiveDate};
use curl::easy::Easy;
use std::fs;
use std::fs::File;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;

mod day1;
mod day2;
mod day3;

const TOKEN: &str = "***REMOVED***";

fn make_day(date: NaiveDate) -> std::io::Result<()> {
    let mut day_dir = PathBuf::from("./src/");
    day_dir.push(format!("day{}", date.day()));

    let url = format!(
        "https://adventofcode.com/{}/day/{}/input",
        date.year(),
        date.day()
    );

    match fs::create_dir(&day_dir) {
        Ok(_) => (),
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => (),
            other_error => {
                panic!("Problem creating directory: {:?}", other_error)
            }
        },
    }

    let mut input_path = day_dir.clone();
    input_path.push("input.txt");

    if !input_path.exists() {
        let mut file = File::create(&input_path)?;

        let mut easy = Easy::new();
        easy.cookie(&format!("session={}", TOKEN)).unwrap();
        easy.url(&url).unwrap();
        easy.write_function(move |data| {
            file.write_all(data).unwrap();
            Ok(data.len())
        })
        .unwrap();
        easy.get(true).unwrap();
        easy.perform()
            .unwrap_or_else(|_| panic!("Encountered error when performing request to {:?}", &url));
        assert_eq!(easy.response_code().unwrap(), 200);
    }

    let mut rs_path = day_dir.clone();
    rs_path.push("mod.rs");

    if !rs_path.exists() {
        let _ = File::create(&rs_path)?;
    }

    Ok(())
}

fn make_some_day(year: i32, day: u32) -> std::io::Result<()> {
    let day = NaiveDate::from_ymd_opt(year, 12, day).expect("should be a valid date");
    make_day(day)
}

fn make_until_today() -> std::io::Result<()> {
    let today = Local::now().day();
    (1..today + 1).try_for_each(|x| make_some_day(2022, x))
}

fn main() -> std::io::Result<()> {
    make_until_today()?;
    day1::main()?;
    day2::main()?;
    day3::main()?;

    Ok(())
}
