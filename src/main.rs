use serde::Deserialize;
use serde::Serialize;
use serde_json;
use reqwest;
use std::io;
use std::io::*;
use std::ops::Add;
use std::collections::HashMap;

const API_ROOT: &str = "https://holidayapi.com/v1/holidays";
const YEAR: &str = "2019";
const API_KEY: &str = "4375217a-5fd6-4b2a-8258-9b5bf543b7cc";
const DEFAULT_COUNTRY_CODE: &str = "US";
// const API_ROOT: &str = "https://holidayapi.com/v1/holidays?key=4375217a-5fd6-4b2a-8258-9b5bf543b7cc&country=US&year=2019";

#[derive(Serialize,Deserialize, Debug)]
struct HolidayApiResponseBody {
    holidays: Vec<Holiday>,
    requests: RequestsInfo,
    status: i32,
    warning: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Holiday {
    country: String,
    date: String,
    name: String,
    observed: String,
    public: bool,
    uuid: String,
    weekday: WeekdayInfo,
}

#[derive(Serialize, Deserialize, Debug)]
struct WeekdayInfo {
    date: WeekdaySpecifics,
    observed: WeekdaySpecifics
}

#[derive(Serialize, Deserialize, Debug)]
struct WeekdaySpecifics {
    name: String,
    // todo: change this to char?
    numeric: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestsInfo {
    available: i32,
    resets: String,
    used: i32
}



fn main() -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let mut query_params = HashMap::new();
    query_params.insert("key", API_KEY);
    query_params.insert("country", DEFAULT_COUNTRY_CODE);
    query_params.insert("year", YEAR);
    let response = client.get(API_ROOT).query(&query_params).send().unwrap();
    let body: HolidayApiResponseBody = response.json().unwrap();
    println!("{}", serde_json::to_string_pretty(&body).unwrap());

    Ok(())
}

fn get_input() {
    // println!("Input a country code or hit enter to use the default (US): ");
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("Failed to parse input");
    //
    // if input.trim().len() < 1 {
    //
    // }
    // println!("INPUT: {}",input);
}


