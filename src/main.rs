/*
Copyright (c) 2024 Collin Ogren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

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
