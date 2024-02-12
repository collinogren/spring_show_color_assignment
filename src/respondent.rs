use std::fs;

#[derive(Default, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Respondent {
    time: u64,
    email: String,
    name: String,
    phone_number: String,
    color1: String,
    color2: String,
    color3: String,
}

impl Respondent {
    pub fn get_color1(&self) -> &String {
        &self.color1
    }

    pub fn get_color2(&self) -> &String {
        &self.color2
    }

    pub fn get_color3(&self) -> &String {
        &self.color3
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
}

const MONTHS_TO_SEC: f64 = 2628333.3333333333333333333333333;
const DAYS_TO_SEC: f64 = 86400.0;
const YEARS_TO_SEC: f64 = 756960000.0;
const HOURS_TO_SEC: f64 = 3600.0;
const MINUTES_TO_SEC: f64 = 60.0;

fn parse_time(input_time: String) -> u64 {
    println!("{}", input_time);
    let mut full_time = input_time.split("/");
    let months = full_time.nth(0).unwrap().parse::<f64>().unwrap() * MONTHS_TO_SEC;
    println!("Months = {}", months);
    let days = full_time.nth(0).unwrap().parse::<f64>().unwrap() * DAYS_TO_SEC;
    println!("Days = {}", days);
    let years = full_time.nth(0).unwrap().split(" ").nth(0).unwrap().parse::<f64>().unwrap() * YEARS_TO_SEC;
    println!("Years = {}", years);
    let mut full_time = input_time.split(" ");
    let mut time = full_time.nth(1).unwrap().split(":");
    let hours = time.nth(0).unwrap().parse::<f64>().unwrap() * HOURS_TO_SEC;
    println!("Hours {}", hours);
    let minutes = time.nth(0).unwrap().parse::<f64>().unwrap() * MINUTES_TO_SEC;
    println!("Minutes {}", minutes);
    let seconds = time.nth(0).unwrap().parse::<f64>().unwrap();

    println!("Seconds {}", seconds);

    return (years + months + days + hours + minutes + seconds).round() as u64;
}

pub fn create_data_set(file_name: &str) -> Vec<Respondent> {
    let raw_data: String = fs::read_to_string(format!("{}{}", "./", file_name)).unwrap();
    let mut raw_data_rows = raw_data.split("\n").collect::<Vec<&str>>();
    raw_data_rows.pop();
    let mut respondents = vec![];
    for row in raw_data_rows.iter() {
        let mut respondent = Respondent::default();
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