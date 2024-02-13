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

/*
    Sorts the entries by time, then places each occurrence of the first color choice into an open
    slot if available. If the respondent is not able to be placed into a slot, they will be placed into a new list
    that will be used for the next iteration.
    Next, the second iteration will begin, this time looking at the second choice.
    This process continues until all choices are exhausted.
    In the case of no choice being available, the only remaining choice will be assigned by placing each respondent
    into the first available slot. This last case should not happen but should still be there as a
    contingency.
*/
use crate::respondent;
use crate::respondent::Respondent;

const NUMBER_OF_GROUPS: usize = 4;

/*
    Attempt to place a participant into a group. If the group is full, then sorting will be attempted again in the next
    iteration with the next most favoured color.
    Each iteration represents, in order, the first, second, and third choices.
*/
fn try_placement(group: &mut Vec<Respondent>, respondent: &Respondent, max_respondents: usize) -> bool {
    // Check if the group is full. If it is, do not add another participant and return false.
    if group.len() >= max_respondents {
        return false;
    }

    // Otherwise, add another participant and return true.
    group.push(respondent.clone());
    true
}

/*
    Attempt to place each respondent into a group and if the group is full, place them into a new array to be processed
    in the next iteration using their next choice of color.
*/
fn calculate_iteration(
    mut green: &mut Vec<Respondent>,
    mut blue: &mut Vec<Respondent>,
    mut red: &mut Vec<Respondent>,
    mut purple: &mut Vec<Respondent>,
    max_respondents: usize,
    input: &Vec<Respondent>,
    phase: u8,
) -> Vec<Respondent> {
    // Create an array to store any and all participants who could not be assigned this iteration.
    let mut next_iter = vec![];

    /*
        For every participant in the list of participants, try to place them into a group based on their color choice
        with priority going to those who responded earlier. For example, suppose that four people already chose green
        as their first choice, two more people do so, only one of those two can go into the green group and so the
        one who responded first gets the spot and the participant who could not get into their first choice will
        be passed to the second iteration to be potentially sorted and so on. All assignments are by chronological
        order in all cases.
    */
    for respondent in input.iter() {
        let color = match phase {
            0 => respondent.get_color1(), // if in the first phase, get the primary choice.
            1 => respondent.get_color2(), // if in the second phase, get the secondary choice.
            2 => respondent.get_color3(), // if in the third phase, get the tertiary choice.
            3 => respondent.get_color4(), // if in the fourth phase, get quaternary choice.
            _ => continue, // This should never happen. Ignore this.
        };

        // Try a placement depending on the color choice.
        match color.as_str() {
            // If they chose greens for this level of choice, then try to place them into the greens group.
            respondent::GREENS => {
                if try_placement(&mut green, respondent, max_respondents) {
                    continue;
                }
            },
            respondent::BLUES => {
                // If they chose blues for this level of choice, then try to place them into the blues group.
                if try_placement(&mut blue, respondent, max_respondents) {
                    continue;
                }
            },
            respondent::REDS => {
                // If they chose reds + oranges for this level of choice, then try to place them into the reds + oranges group.
                if try_placement(&mut red, respondent, max_respondents) {
                    continue;
                }
            },
            respondent::PURPLES => {
                // If they chose pinks + purples for this level of choice, then try to place them into the pinks + purples group.
                if try_placement(&mut purple, respondent, max_respondents) {
                    continue;
                }
            },
            &_ => {},
        }

        // If no assignment was made, then they need to go through the process again,
        // this time using one step less favourable of a color choice.
        next_iter.push(respondent.clone());
    }

    sort_chronologically(&mut next_iter);
    next_iter // Return a chronologically sorted array of all the participants who did not get assigned.
}

/*
    Sort by chronological order, calculate maximum group size which is (the number of participants / the number of groups) + 1,
    create arrays for each color group and pass them to the calculate_iteration() function which returns a list of all
    unsorted participants. calculate_iteration() is called three times so that the first, second, and third choices are all considered (if needed).
*/
pub fn calculate_colors(mut respondents: Vec<Respondent>) -> (Vec<Respondent>, Vec<Respondent>, Vec<Respondent>, Vec<Respondent>) {
    sort_chronologically(&mut respondents);

    // Maximum size of each group. the + 1 handles remainder.
    let max_respondents = (respondents.len() / NUMBER_OF_GROUPS) + 1;

    // Create arrays to represent each group.
    let mut green = vec![];
    let mut blue = vec![];
    let mut red = vec![];
    let mut purple = vec![];

    // First iteration.
    let next_iter = calculate_iteration(&mut green, &mut blue, &mut red, &mut purple, max_respondents, &respondents, 0);

    // Second iteration.
    let next_iter = calculate_iteration(&mut green, &mut blue, &mut red, &mut purple, max_respondents, &next_iter, 1);

    // Third iteration.
    let next_iter = calculate_iteration(&mut green, &mut blue, &mut red, &mut purple, max_respondents, &next_iter, 2);

    // Final iteration if all else fails.
    calculate_iteration(&mut green, &mut blue, &mut red, &mut purple, max_respondents, &next_iter, 3);

    // Return groups with assignments.
    (green, blue, red, purple)
}

// Sorts an array of respondents chronologically.
fn sort_chronologically(respondents: &mut Vec<Respondent>) {
    respondents.sort_by(|a, b| b.get_time().cmp(&a.get_time()));
    respondents.reverse();
}