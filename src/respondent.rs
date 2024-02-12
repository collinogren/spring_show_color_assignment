use std::fs;

#[derive(Default, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Respondent {
    email: String,
    name: String,
    color1: String,
    color2: String,
    color3: String,
    time: u64,
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
}

const MONTHS_TO_SEC: f64 = 2628333.3333333333333333333333333;
const DAYS_TO_SEC: f64 = 86400.0;
const YEARS_TO_SEC: f64 = 756960000.0;
const HOURS_TO_SEC: f64 = 3600.0;
const MINUTES_TO_SEC: f64 = 60.0;

fn parse_time(full_time: String) -> u64 {
    let mut full_time = full_time.split("/");
    let months = full_time.nth(0).unwrap().parse::<f64>().unwrap() * MONTHS_TO_SEC;
    let days = full_time.nth(1).unwrap().parse::<f64>().unwrap() * DAYS_TO_SEC;
    let years = full_time.nth(2).unwrap().split(" ").nth(0).unwrap().parse::<f64>().unwrap() * YEARS_TO_SEC;
    let mut time = full_time.nth(2).unwrap().split(" ").nth(1).unwrap().split(":");
    let hours = time.nth(0).unwrap().parse::<f64>().unwrap() * HOURS_TO_SEC;
    let minutes = time.nth(1).unwrap().parse::<f64>().unwrap() * MINUTES_TO_SEC;
    let seconds = time.nth(2).unwrap().parse::<f64>().unwrap();

    return (years + months + days + hours + minutes + seconds).round() as u64;
}

fn create_data_set(file_name: String) -> Vec<Respondent> {
    let raw_data: String = fs::read_to_string(format!("{}{}", "./", file_name)).unwrap();
    let raw_data_rows = raw_data.split("\n").collect::<Vec<&str>>();
    let mut respodents = vec![];
    for row in raw_data_rows.iter() {
        let mut respondent = Respondent::default();
        let respondent_data = row.split("	").collect::<Vec<&str>>();
        for (i, respondent_datum) in respondent_data.iter().enumerate() {
            match i {
                0 => respondent.email = respondent_datum.to_string(),
                1 => respondent.name = respondent_datum.to_string(),
                2 => respondent.color1 = respondent_datum.to_string(),
                3 => respondent.color2 = respondent_datum.to_string(),
                4 => respondent.color3 = respondent_datum.to_string(),
                5 => respondent.time = parse_time(respondent_datum.to_string()),
                _ => {
                    panic!("Too many columns");
                }
            }
        }

        respodents.push(respondent);
    }

    respodents
}