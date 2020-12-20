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
// todo: remove this from source and git history
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

struct HolidayStats<'a> {
    public_holidays: Vec<&'a Holiday>,
    weekday_holidays: Vec<&'a Holiday>,
    weekend_holidays: Vec<&'a Holiday>,
    all_holidays: Vec<Holiday>,
}

impl HolidayStats<'_> {
    fn from(holidays: &[Holiday]) -> HolidayStats {
        let mut stats = HolidayStats {
            // num_public_holidays: 0,
            public_holidays: vec![],
            weekday_holidays: vec![],
            weekend_holidays: vec![],
            all_holidays: vec![],
        };
        for holiday in holidays {
            let holiday_cloned = holiday.clone();
            stats.all_holidays.push(holiday_cloned);
            if holiday.public {
                stats.public_holidays.push(&holiday_cloned);
            }
            if holiday.weekday.date.is_weekend() {
                stats.weekend_holidays.push(&holiday_cloned);
            } else {
                stats.weekday_holidays.push(&holiday_cloned);
            }
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

// todo: all the primary options
// todo: deserialization replace String in the structs with something fixed length?
// todo: deserialization try to rpelace String with dates in data fields
fn main() -> Result<()> {
    // fetch_data();
    let selected_option = get_starting_input();
    // todo:  1) how many holidates in the US 2019
    // todo:  2) how many holidays in a different country in 2019 (prompt user for country code)
    // todo:  3) compare two different countries (prompt user for two country codes)
    // todo:  4) I just want to check if a number is prime
    // todo:  5) Exit
    match selected_option {
        Ok(1) => {
            match fetch_data(DEFAULT_COUNTRY_CODE) {
                Ok(api_data) => {
                    let pretty_json = serde_json::to_string_pretty(&api_data)?;
                    println!("Data received");
                    // println!("data received: \n{}\n", pretty_json);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }

        Ok(2) => {
            println!("2qwerty");
            match get_input("Enter a country code: ") {
                Ok(country_code) => {
                    match fetch_data(&country_code) {
                        Ok(api_data) => {
                            let pretty_json = serde_json::to_string_pretty(&api_data)?;
                            println!("Data received");
                            println!("data received: \n{}\n", pretty_json);
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }

        Ok(3) => {
            println!("3zxc");
        }

        Ok(4) => {
            println!("4yurt");
        }
        Ok(5) => {
            println!("See you later.");
        }
        Ok(_) => {
            println!("Not a valid selection. Exiting.");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
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

// todo: add a menu so you can 'pick' which option you want
// todo options:


//Challenge 1:
// How many public holidays are there in the US for the wonderful year 2019?
//
// Variations:
// •	Break down the results into public holidays that fall on a workday versus a weekend
// •	Allow the user to input a different country code*
//      o	Allow the user to input a different country and get the code
// •	Allow the user to input two country codes and compare the results*
//      o	Which has a higher percentage of public to non-public holidays?
//      o	Which holidays are shared between these countries?
//      o	Which country has more subdivisions?
//      o	Which country has a cooler flag?
//      o	Allow the user to provide more than two countries

// MFR - modify this one... when comparing two countries (see above) , include a comparison of the num of prime number dates
// •	How many workday dates within this timeframe are prime numbers?  Convert the date into an integer by concatenating the date parts in the format DD-MM-YYYY and removing leading zeros.
//      o	Example:
//          	01-01-2019 -> 1012019


fn get_starting_input() -> std::result::Result<i8, Box<dyn Error>> {
    // println!("Input a country code or hit enter to use the default (US): ");
    println!("Enter the corresponding number to select one of the following: ");
    println!("1) I want to know how many holidays there were in the US in 2019");
    println!("2) I want to know how many holidays there were in 2019 in a different country");
    println!("3) I want to compare two different countries' holidays from 2019");
    println!("4) Actually I just want to check if a number is prime.");
    println!("5) Exit.");

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

// {
//   "holidays": [
//     {
//       "country": "DE",
//       "date": "2019-01-01",
//       "name": "New Year's Day",
//       "observed": "2019-01-01",
//       "public": true,
//       "uuid": "4cd0a2a4-fb31-48c5-a305-72f53734c052",
//       "weekday": {
//         "date": {
//           "name": "Tuesday",
//           "numeric": "2"
//         },
//         "observed": {
//           "name": "Tuesday",
//           "numeric": "2"
//         }
//       }
//     },

