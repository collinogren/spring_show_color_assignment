/*
    Sorts the entries by time, then places each occurrence of the first color choice into an open
    slot if available and the respondent will be removed from the list of un-chosen respondents.
    Once the whole list has been iterated over once, the second iteration will begin, this time
    looking at the second choice. This process continues until all choices are exhausted. In the case
    of no choice being available, the only remaining choice will be assigned by placing each respondent
    into the first available slot. This last case should not happen but should still be there as a
    contingency.
*/
use std::ops::Sub;
use crate::respondent::Respondent;

const NUMBER_OF_GROUPS: usize = 4;

fn try_placement(color: &mut Vec<Respondent>, respondent: &Respondent, max_respondents: usize, remainder_respondents: &mut usize) -> bool {
    if color.len() >= max_respondents {
        if *remainder_respondents <= 0usize {
            return false;
        } else {
            remainder_respondents.sub(1);
        }
    }

    color.push(respondent.clone());
    true
}

fn calculate_iteration(
    mut green: &mut Vec<Respondent>,
    mut blue: &mut Vec<Respondent>,
    mut red: &mut Vec<Respondent>,
    mut purple: &mut Vec<Respondent>,
    max_respondents: usize,
    remainder_respondents: &mut usize,
    input: &Vec<Respondent>,
    phase: u8,
) -> Vec<Respondent> {
    let mut next_iter = vec![];
    for respondent in input.iter() {
        let color = match phase {
            0 => respondent.get_color1(),
            1 => respondent.get_color2(),
            2 => respondent.get_color3(),
            _ => continue,
        };
        match color.as_str() {
            "green" => {
                if try_placement(&mut green, respondent, max_respondents, remainder_respondents) {
                    continue;
                }
            },
            "blue" => {
                if try_placement(&mut blue, respondent, max_respondents, remainder_respondents) {
                    continue;
                }
            },
            "red" => {
                if try_placement(&mut red, respondent, max_respondents, remainder_respondents) {
                    continue;
                }
            },
            "purple" => {
                if try_placement(&mut purple, respondent, max_respondents, remainder_respondents) {
                    continue;
                }
            },
            &_ => {},
        }

        next_iter.push(respondent.clone());
    }

    next_iter.sort_by(|a, b| b.get_time().cmp(&a.get_time()));
    next_iter
}

fn calculate_colors(mut respondents: Vec<Respondent>) {
    respondents.sort_by(|a, b| b.get_time().cmp(&a.get_time()));
    let max_respondents = respondents.len() / NUMBER_OF_GROUPS;
    let mut remainder_respondents = respondents.len() % NUMBER_OF_GROUPS;
    let mut green = vec![];
    let mut blue = vec![];
    let mut red = vec![];
    let mut purple = vec![];

    let next_iter = calculate_iteration(&mut green, &mut blue, &mut red, &mut purple, max_respondents, &mut remainder_respondents, &respondents, 0);
    let next_iter = calculate_iteration(&mut green, &mut blue, &mut red, &mut purple, max_respondents, &mut remainder_respondents, &next_iter, 1);
    calculate_iteration(&mut green, &mut blue, &mut red, &mut purple, max_respondents, &mut remainder_respondents, &next_iter, 2);
}