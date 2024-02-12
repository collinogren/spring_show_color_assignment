/*
    Sorts the entries by time, then places each occurrence of the first color choice into an open
    slot if available and the respondent will be removed from the list of un-chosen respondents.
    Once the whole list has been iterated over once, the second iteration will begin, this time
    looking at the second choice. This process continues until all choices are exhausted. In the case
    of no choice being available, the only remaining choice will be assigned by placing each respondent
    into the first available slot. This last case should not happen but should still be there as a
    contingency.
*/
use crate::respondent::Respondent;

const NUMBER_OF_GROUPS: usize = 4;

// Attempt to place a participant into a group. If the group is full, then that means it must be sorted in the next iteration.
// Each iteration represents, in order, the first, second, and third choices.
fn try_placement(group: &mut Vec<Respondent>, respondent: &Respondent, max_respondents: usize) -> bool {
    println!("{}", respondent.get_name());
    if group.len() >= max_respondents {
        return false;
    }

    group.push(respondent.clone());
    true
}

// Attempt to place each respondent into a group and if the group is full, place them into a new array to be processed
// in the next iteration using their next choice of color.
fn calculate_iteration(
    mut green: &mut Vec<Respondent>,
    mut blue: &mut Vec<Respondent>,
    mut red: &mut Vec<Respondent>,
    mut purple: &mut Vec<Respondent>,
    max_respondents: usize,
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
            "Greens" => {
                if try_placement(&mut green, respondent, max_respondents) {
                    continue;
                }
            },
            "Blues" => {
                if try_placement(&mut blue, respondent, max_respondents) {
                    continue;
                }
            },
            "Reds + Oranges" => {
                if try_placement(&mut red, respondent, max_respondents) {
                    continue;
                }
            },
            "Pinks + Purples" => {
                if try_placement(&mut purple, respondent, max_respondents) {
                    continue;
                }
            },
            &_ => {},
        }

        next_iter.push(respondent.clone());
    }

    next_iter.sort_by(|a, b| b.get_time().cmp(&a.get_time()));
    next_iter.reverse();
    next_iter
}

// Sort by chronological order, calculate maximum group size which is (the number of participants / the number of groups) + 1,
// create arrays for each color group and pass them to the calculate_iteration() function which returns a list of all
// unsorted participants. calculate_iteration() is called three times so that the first, second, and third choices are all considered (if needed).
pub fn calculate_colors(mut respondents: Vec<Respondent>) -> (Vec<Respondent>, Vec<Respondent>, Vec<Respondent>, Vec<Respondent>) {
    respondents.sort_by(|a, b| b.get_time().cmp(&a.get_time()));
    respondents.reverse();
    let max_respondents = (respondents.len() / NUMBER_OF_GROUPS) + 1;
    let mut green = vec![];
    let mut blue = vec![];
    let mut red = vec![];
    let mut purple = vec![];

    let next_iter = calculate_iteration(&mut green, &mut blue, &mut red, &mut purple, max_respondents, &respondents, 0);
    let next_iter = calculate_iteration(&mut green, &mut blue, &mut red, &mut purple, max_respondents, &next_iter, 1);
    calculate_iteration(&mut green, &mut blue, &mut red, &mut purple, max_respondents, &next_iter, 2);

    (green, blue, red, purple)
}