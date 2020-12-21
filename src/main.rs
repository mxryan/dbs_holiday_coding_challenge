use serde::Deserialize;
use serde::Serialize;
use serde_json;
use reqwest;
use std::io;
use std::io::*;
use std::collections::HashMap;
use std::error::Error;
use primes;

const API_ROOT: &str = "https://holidayapi.com/v1/holidays";
const YEAR: &str = "2019";
const API_KEY: &str = "4375217a-5fd6-4b2a-8258-9b5bf543b7cc";
const ASK_FOR_COUNTRY_CODES_MSG: &str = "Enter country codes separated by spaces. A maximum of 3 country codes are supported at this time";

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

#[derive(Debug)]
struct HolidayStats {
    public_holidays: Vec<usize>,
    weekday_holidays: Vec<usize>,
    weekend_holidays: Vec<usize>,
    all_holidays: Vec<Holiday>,
    country_code: String,
}

impl HolidayStats {
    fn from(holidays: &[Holiday], country_code: &str) -> HolidayStats {
        let mut stats = HolidayStats {
            public_holidays: vec![],
            weekday_holidays: vec![],
            weekend_holidays: vec![],
            all_holidays: vec![],
            country_code: String::from(country_code),
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
        self.weekday_holidays.len()
    }

    fn get_num_weekend(&self) -> usize {
        self.weekend_holidays.len()
    }

    fn get_num_prime_holidays(&self) -> i32 {
        self.all_holidays.iter().fold(0, |acc, holiday| {
            if is_date_prime(&holiday.date) {
                acc + 1
            } else {
                acc
            }
        })
    }

    fn print_descriptive_stats(&self) {
        println!("===== ===== country: {} ===== =====", self.country_code);
        println!(" public holidays: {}", self.get_num_public());
        println!("weekday holidays: {}", self.get_num_weekday());
        println!("weekend holidays: {}", self.get_num_weekend());
        println!("  prime holidays: {}", self.get_num_prime_holidays());
        println!("===== ===== ===== ===== ===== =====");
        println!(" ");
    }
}

fn main() -> Result<()> {
    match get_country_inputs() {
        Ok(country_codes) => {
            for code in country_codes {
                match fetch_data(&code) {
                    Ok(data) => display_stats(&code, &data),
                    Err(e) => println!("Fetch failed ({}), error: {}", code, e)
                }
            }
        }
        Err(e) => println!("Error: {}", e)
    }
    Ok(())
}

fn get_country_inputs() -> std::result::Result<Vec<String>, Box<dyn Error>> {
    let user_input = get_input(ASK_FOR_COUNTRY_CODES_MSG)?;
    let mut codes: Vec<String> = user_input
        .trim()
        .split_whitespace()
        .map(|x| String::from(x.trim()))
        .collect();

    if codes.len() > 3 {
        println!("Only three countries are supported at this time");
        while codes.len() > 3 {
            codes.pop();
        }
    }

    Ok(codes)
}

fn get_input(msg: &str) -> std::result::Result<String, Box<dyn Error>> {
    if msg.len() > 0 {
        println!("{}", msg)
    }
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(String::from(input.trim()))
}

fn fetch_data(country_code: &str)
              -> std::result::Result<HolidayApiResponseBody, Box<dyn Error>>
{
    let client = reqwest::blocking::Client::new();
    let query_params = build_query_map(country_code);
    let response = client.get(API_ROOT).query(&query_params).send()?;

    Ok(response.json()?)
}

fn build_query_map(country_code: &str) -> HashMap<&str, &str> {
    let mut query_params = HashMap::new();
    query_params.insert("key", API_KEY);
    query_params.insert("country", country_code);
    query_params.insert("year", YEAR);
    query_params
}

fn display_stats(code: &String, data: &HolidayApiResponseBody) {
    let stats = HolidayStats::from(&data.holidays, &code);
    stats.print_descriptive_stats();
}

fn is_date_prime(date_yyyy_mm_dd: &str) -> bool {
    let x: Vec<&str> = date_yyyy_mm_dd.split("-").collect();
    let date_chunks: Vec<&str> = x.iter().rev().map(|chunk| *chunk).collect();
    let num: u64 = date_chunks.join("").parse().unwrap();
    primes::is_prime(num)
}
