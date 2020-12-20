use serde::Deserialize;
use serde::Serialize;
use serde_json;
use reqwest;
use std::io;
use std::io::*;
use std::ops::Add;
use std::collections::HashMap;
use std::error::Error;

const API_ROOT: &str = "https://holidayapi.com/v1/holidays";
const YEAR: &str = "2019";
const API_KEY: &str = "4375217a-5fd6-4b2a-8258-9b5bf543b7cc";
const DEFAULT_COUNTRY_CODE: &str = "US";

#[derive(Serialize, Deserialize, Debug)]
struct HolidayApiResponseBody {
    holidays: Vec<Holiday>,
    requests: RequestsInfo,
    status: i32,
    warning: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Holiday {
    country: String,
    date: String,
    name: String,
    observed: String,
    public: bool,
    uuid: String,
    weekday: WeekdayInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct WeekdayInfo {
    date: WeekdaySpecifics,
    observed: WeekdaySpecifics,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct WeekdaySpecifics {
    name: String,
    numeric: char,
}

impl WeekdaySpecifics {
    fn is_weekend(&self) -> bool {
        self.numeric == '6' || self.numeric == '7'
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestsInfo {
    available: i32,
    resets: String,
    used: i32,
}

struct HolidayStats {
    public_holidays: Vec<usize>,
    weekday_holidays: Vec<usize>,
    weekend_holidays: Vec<usize>,
    all_holidays: Vec<Holiday>,
}

impl HolidayStats {
    fn from(holidays: &[Holiday]) -> HolidayStats {
        let mut stats = HolidayStats {
            public_holidays: vec![],
            weekday_holidays: vec![],
            weekend_holidays: vec![],
            all_holidays: vec![],
        };
        for i in 0..holidays.len() {
            let holiday_cloned = holidays[i].clone();
            if holiday_cloned.public {
                stats.public_holidays.push(i);
            }
            if holiday_cloned.weekday.date.is_weekend() {
                stats.weekend_holidays.push(i);
            } else {
                stats.weekday_holidays.push(i);
            }
            stats.all_holidays.push(holiday_cloned);
        }
        stats
    }

    fn get_num_public(&self) -> usize {
        self.public_holidays.len()
    }

    fn get_num_weekday(&self) -> usize {
        self.weekend_holidays.len()
    }

    fn get_num_weekend(&self) -> usize {
        self.weekend_holidays.len()
    }
}

fn pretty_print_json(api_data: &HolidayApiResponseBody) {
    let pretty_json = serde_json::to_string_pretty(&api_data)
        .expect("Failed to serialize to JSON for pretty-printing.");
    println!("data received: \n{}\n", pretty_json);
}

fn main() -> Result<()> {
    let selected_option = get_starting_input();
    match selected_option {
        Ok(1) => {
            match get_input("Enter a country code: ") {
                Ok(country_code) => {
                    match fetch_data(&country_code) {
                        Ok(api_data) => println!("Data received"),
                        Err(e) => println!("Error: {}", e),
                    }
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        Ok(2) => println!("TODO: ENTER TWO COUNTRY CODES"),
        Ok(3) => println!("See you later."),
        Ok(_) => println!("Not a valid selection. Exiting."),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

// todo: test the response when you run this with junk country code
fn fetch_data(country_code: &str)
              -> std::result::Result<HolidayApiResponseBody, Box<dyn Error>> {
    println!("Fetching data for {}...", country_code);
    let client = reqwest::blocking::Client::new();
    let mut query_params = HashMap::new();
    query_params.insert("key", API_KEY);
    query_params.insert("country", country_code);
    query_params.insert("year", YEAR);
    let response = client.get(API_ROOT).query(&query_params).send()?;
    let body: HolidayApiResponseBody = response.json()?;
    Ok(body)
}

fn get_starting_input() -> std::result::Result<i8, Box<dyn Error>> {
    println!("Select one of the following: ");
    println!("1) I want to know how many holidays there were in 2019 in a country");
    println!("2) I want to compare two different countries' holidays from 2019");
    println!("3) Exit.");
    println!("Enter selection: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let result: i8 = input.trim().parse()?;

    Ok(result)
}

fn get_input(msg: &str) -> std::result::Result<String, Box<dyn Error>> {
    if msg.len() > 0 {
        println!("{}", msg)
    }
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(String::from(input.trim()))
}

