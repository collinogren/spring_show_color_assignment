use std::fs;
use native_dialog::FileDialog;
use crate::calculator::calculate_colors;
use crate::respondent::{create_data_set, Respondent};

mod respondent;
mod calculator;

// Create output files.
fn print(resp: Vec<Respondent>, color: &str, output_name: &str) {
    let mut output = String::new();
    for r in &resp {
        output = format!("{}\n{} {} {} {}", output, r.get_email(), r.get_name(), r.get_phone_number(), color);
    }

    output.remove(0);

    fs::write(format!("./{}", output_name), output).unwrap();
}

fn get_path_from_user() -> (String, bool) {
    let path = FileDialog::new().show_open_single_file();
    match path {
        Ok(v) => {(v.unwrap().to_str().unwrap().to_string(), true)},
        Err(_) => {(String::new(), false)},
    }
}

fn main() {
    let (path, success) = get_path_from_user();

    if !success {
        return;
    }

    let data = create_data_set(path.as_str());
    let (green, blue, red, purple) = calculate_colors(data);

    print(green, "Greens", "Greens.txt");

    print(blue, "Blues", "Blues.txt");

    print(red, "Reds+Oranges", "Reds + Oranges.txt");

    print(purple, "Pinks+Purples", "Pinks + Purples.txt");
}
