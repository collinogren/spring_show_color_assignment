use std::fs;

/*
    This file contains a representation of the response data and a parser to bring the data into the program.
*/

pub const GREENS: &'static str = "Greens";
pub const BLUES: &'static str = "Blues";
pub const REDS: &'static str = "Reds + Oranges";
pub const PURPLES: &'static str = "Pinks + Purples";

#[derive(Default, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Respondent {
    time: u64,
    email: String,
    name: String,
    phone_number: String,
    color1: String,
    color2: String,
    color3: String,
    color4: String,
}

impl Respondent {
    pub fn new() -> Self {
        let mut respondent = Self::default();
        respondent.set_remaining_color();
        respondent
    }
    pub fn get_color1(&self) -> &String {
        &self.color1
    }

    pub fn get_color2(&self) -> &String {
        &self.color2
    }

    pub fn get_color3(&self) -> &String {
        &self.color3
    }

    pub fn get_color4(&self) -> &String {
        &self.color4
    }

    pub fn get_time(&self) -> u64 {
        self.time.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_email(&self) -> String {
        self.email.clone()
    }

    pub fn get_phone_number(&self) -> String {
        self.phone_number.clone()
    }

    fn set_color4(&mut self, s: String) {
        self.color4 = s;
    }

    // Just see which color was not chosen at all and assign that to color4.
    fn set_remaining_color(&mut self) {
        if self.get_color1() != GREENS && self.get_color2() != GREENS && self.get_color3() != GREENS {
            self.set_color4(GREENS.to_string());
        }

        if self.get_color1() != BLUES && self.get_color2() != BLUES && self.get_color3() != BLUES {
            self.set_color4(BLUES.to_string());
        }

        if self.get_color1() != REDS && self.get_color2() != REDS && self.get_color3() != REDS {
            self.set_color4(REDS.to_string());
        }

        if self.get_color1() != PURPLES && self.get_color2() != PURPLES && self.get_color3() != PURPLES {
            self.set_color4(PURPLES.to_string());
        }
    }
}

const MONTHS_TO_SEC: f64 = 2628333.3333333333333333333333333;
const DAYS_TO_SEC: f64 = 86400.0;
const YEARS_TO_SEC: f64 = 756960000.0;
const HOURS_TO_SEC: f64 = 3600.0;
const MINUTES_TO_SEC: f64 = 60.0;

// Parses date and time data into seconds since the non-existent year 0. Why year 0? Because I didn't feel like making
// things more complicated just to humour the ridiculous notion people have about starting things at 1 instead of 0
// even when they definitely should. Looking at you AWK, COBOL, Fortran, R, Julia, Lua, MATLAB, Smalltalk, Wolfram,
// and any others who may apply!
fn parse_time(input_time: String) -> u64 {
    let mut full_time = input_time.split("/");
    let months = full_time.nth(0).unwrap().parse::<f64>().unwrap() * MONTHS_TO_SEC;
    let days = full_time.nth(0).unwrap().parse::<f64>().unwrap() * DAYS_TO_SEC;
    let years = full_time.nth(0).unwrap().split(" ").nth(0).unwrap().parse::<f64>().unwrap() * YEARS_TO_SEC;
    let mut full_time = input_time.split(" ");
    let mut time = full_time.nth(1).unwrap().split(":");
    let hours = time.nth(0).unwrap().parse::<f64>().unwrap() * HOURS_TO_SEC;
    let minutes = time.nth(0).unwrap().parse::<f64>().unwrap() * MINUTES_TO_SEC;
    let seconds = time.nth(0).unwrap().parse::<f64>().unwrap();

    return (years + months + days + hours + minutes + seconds).round() as u64;
}

/*
    Parse each row of data into an instance of the Respondent struct, then place into an array to be returned.
*/
pub fn create_data_set(path: &str) -> Vec<Respondent> {
    let raw_data: String = fs::read_to_string(path).unwrap();
    let mut raw_data_rows = raw_data.split("\n").collect::<Vec<&str>>();
    raw_data_rows.pop();
    let mut respondents = vec![];
    for row in raw_data_rows.iter() {
        let mut respondent = Respondent::new();
        let respondent_data = row.split("	").collect::<Vec<&str>>();
        for (i, respondent_datum) in respondent_data.iter().enumerate() {
            match i {
                0 => respondent.time = parse_time(respondent_datum.to_string()),
                1 => respondent.email = respondent_datum.to_string(),
                2 => respondent.name = respondent_datum.to_string(),
                3 => respondent.phone_number = respondent_datum.to_string(),
                4 => respondent.color1 = respondent_datum.to_string(),
                5 => respondent.color2 = respondent_datum.to_string(),
                6 => respondent.color3 = respondent_datum.to_string(),
                _ => {
                    panic!("Too many columns");
                }
            }
        }

        respondents.push(respondent);
    }

    respondents
}