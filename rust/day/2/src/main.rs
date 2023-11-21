// A == Rock
// B == Paper
// C == Scissors

// X == Rock
// Y == Paper
// Z == Scissors

// 1P for Rock
// 2P for Paper
// 3P for Scissors

// 0P for lost
// 3P for draw
// 6P for win

fn print_total_points_1() {
    let mut total_points = 0;
    let file = std::fs::read_to_string("input")
        .unwrap();
    file.lines().for_each(|line| {
        let mut points = 0;
        let vec: Vec<_> = line.split_whitespace().collect();
        let op = vec.first().unwrap();
        let &ply = vec.last().unwrap();
        match &ply[..] {
            "X" => points += 1, // Rock
            "Y" => points += 2, // Paper
            "Z" => points += 3, // Scissors
            _ => unreachable!()
        }
        
        match &op[..] {
            "A" => { // Rock
                if points == 1 {
                    points += 3 // tie
                } else if points == 2 {
                    points += 6 // win
                }
            }
            "B" => { // Paper
                if points == 2 {
                    points += 3 // tie
                } else if points == 3 {
                    points += 6 // win
                }
            }
            "C" => { // Scissors
                if points == 3 {
                    points += 3 // tie
                } else if points == 1 {
                    points += 6
                }
            }
            _ => unreachable!()
        }
        total_points += points;
    });
    println!("{}", total_points)
}

// X means you need to loose
// Y means you need to draw
// Z means you need to win
enum RPS {
    Rock,
    Paper,
    Scissors
}

fn get_rps(x: &&str) -> RPS {
    match &x[..] {
        "A" => return RPS::Rock,
        "B" => return RPS::Paper,
        "C" => return RPS::Scissors,
        _ => unreachable!()
    }
}

fn get_rps_from_outcome(x: RPS, y: &Outcome) -> RPS {
    match y {
        Outcome::Loose => {
            match x {
                RPS::Rock => return RPS::Scissors,
                RPS::Paper => return RPS::Rock,
                RPS::Scissors => return RPS::Paper,
            }
        },
        Outcome::Draw => {
            return x;
        },
        Outcome::Win => {
            match x {
                RPS::Rock => return RPS::Paper,
                RPS::Paper => return RPS::Scissors,
                RPS::Scissors => return RPS::Rock,
            }
        },
    }
}

enum Outcome {
    Loose,
    Draw,
    Win
}

fn get_outcome(x: &&str) -> Outcome {
    match &x[..] {
        "X" => return Outcome::Loose,
        "Y" => return Outcome::Draw,
        "Z" => return Outcome::Win,
        _ => unreachable!()
    }
}

fn print_total_points_2() {
    let mut total_points = 0;
    let file = std::fs::read_to_string("input")
        .unwrap();
    file.lines().for_each(|line| {
        let mut points = 0;
        let vec: Vec<_> = line.split_whitespace().collect();
        let op = get_rps(vec.first().unwrap());
        let out = get_outcome(vec.last().unwrap());
        let ply = get_rps_from_outcome(op, &out);

        match ply {
            RPS::Rock => points += 1,
            RPS::Paper => points += 2,
            RPS::Scissors => points += 3,
        }

        match &out {
            Outcome::Loose => points += 0,
            Outcome::Draw => points += 3,
            Outcome::Win => points += 6,
        }
        
        total_points += points;
    });
    println!("{}", total_points)
}

fn main() {
    println!("Hello, world!");
    print_total_points_1();
    print_total_points_2();
}
