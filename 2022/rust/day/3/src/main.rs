// To help prioritize item rearrangement, every item type can be converted to a priority:
// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
// In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.

use std::collections::HashSet;

fn convert_letter_to_points(c: char) -> u32 {
    let mut num = 0;
    let y = c.to_digit(36).unwrap() - 9;
    num += y;
    if c.is_uppercase() {
        num += 26
    }
    return num;
}

fn part_1() {
    let file = std::fs::read_to_string("input")
        .unwrap();

    let mut foo = 0;
    file.lines().for_each(|line| {
        let mut set: HashSet<u32> = HashSet::new();
        let mut score_added = false;
        line.chars().enumerate().for_each(|(idx, c)| {
            if score_added == false {
                let num = convert_letter_to_points(c);
                if idx + 1 <= line.len() / 2 {
                    set.insert(num);
                } else {
                    let x = set.get(&num);
                    if x.is_some() {
                        foo += num;
                        score_added = true
                    }
                }
            }
        });
    });
    println!("{}", foo)
}

fn part_2() {
    let file = std::fs::read_to_string("input")
        .expect("input file should exist");

    let mut set_x: HashSet<u32> = HashSet::new();
    let mut set_y: HashSet<u32> = HashSet::new();
    let mut foo = 0;
    let mut score_added = false;
    file.lines().enumerate().for_each(|(idx, line)| {
        match idx % 3 {
            0 => {
                if set_x.len() != 0 {
                    set_x.clear();
                }
                if score_added == true {
                    score_added = false;
                }
                line.chars().for_each(|c| {
                    let x = convert_letter_to_points(c);
                    set_x.insert(x);
                });
            },
            1 => {
                if set_y.len() != 0 {
                    set_y.clear();
                }
                line.chars().for_each(|c| {
                    let y = convert_letter_to_points(c);
                    if let Some(x) = set_x.get(&y) {
                        set_y.insert(*x);
                    }
                });
            },
            2 => {
                let mut count = 0;
                line.chars().for_each(|c| {
                    if score_added == false {
                        let z = convert_letter_to_points(c);
                        if let Some(y) = set_y.get(&z) {
                            count += 1;
                            foo += y;
                            score_added = true;
                        }
                    }
                });
            },
            _ => unreachable!()
        }
    });
    println!("{}", foo);
}

fn main() {
    part_1();
    part_2();
}
