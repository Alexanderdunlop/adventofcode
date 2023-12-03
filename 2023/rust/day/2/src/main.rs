#[derive(Debug)]
struct Combination {
    blue: usize,
    red: usize,
    green: usize,
}

fn part1(file: String, possible: Combination) -> i32 {
    let mut foo = 0;
    file
        .lines()
        .for_each(|line| {
            let (a, b) = line.split_once(": ").unwrap();
            let mut too_many = false;
            b.split("; ").for_each(|pull| {
                let items = pull.split(", ");
                items.for_each(|item| {
                    let (value, color) = item.split_once(" ").unwrap();
                    let x = value.parse::<usize>().unwrap();
                    match color {
                        "blue" => {
                            if x > possible.blue {
                                too_many = true;
                            }
                        },
                        "red" => {
                            if x > possible.red {
                                too_many = true;
                            }
                        },
                        "green" => {
                            if x > possible.green {
                                too_many = true;
                            }
                        },
                        _ => unimplemented!()
                    }
                })
            });
            if !too_many {
                let (_, game) = a.split_once(" ").unwrap();
                foo += game.parse::<i32>().unwrap()
            }
        });
    return foo;
}

fn part2(file: String) -> usize {
    let mut foo = 0;
    file
        .lines()
        .for_each(|line| {
            let (_, b) = line.split_once(": ").unwrap();
            let mut possible_cubes = Combination {
                blue: 0,
                red: 0,
                green: 0
            };
            b.split("; ").for_each(|pull| {
                let items = pull.split(", ");
                items.for_each(|item| {
                    let (value, color) = item.split_once(" ").unwrap();
                    let x = value.parse::<usize>().unwrap();
                    match color {
                        "blue" => {
                            if possible_cubes.blue < x {
                                possible_cubes.blue = x
                            }
                        },
                        "red" => {
                            if possible_cubes.red < x {
                                possible_cubes.red = x
                            }
                        },
                        "green" => {
                            if possible_cubes.green < x {
                                possible_cubes.green = x
                            }
                        },
                        _ => unimplemented!()
                    }
                })
            });
            foo += possible_cubes.blue * possible_cubes.red * possible_cubes.green;
        });
    return foo;
}

fn part1_main() {
    let test_file = std::fs::read_to_string("test-input")
        .unwrap();
    let test_combination = Combination {
        blue: 14,
        red: 12,
        green: 13
    };
    let test_result = part1(test_file, test_combination);
    assert_eq!(test_result, 8);
    let file = std::fs::read_to_string("input")
        .unwrap();
    let combination = Combination {
        blue: 14,
        red: 12,
        green: 13,
    };
    let result = part1(file, combination);
    println!("{}", result);
}

fn part2_main() {
    let test_file = std::fs::read_to_string("test-input-2")
        .unwrap();
    let test_result = part2(test_file);
    assert_eq!(test_result, 2286);
    let file = std::fs::read_to_string("input").unwrap();
    let result = part2(file);
    println!("{}", result);
}

fn main() {
    part1_main();
    part2_main();
}
