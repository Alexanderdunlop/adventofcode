// Some of the pairs have noticed that one of their assignments fully contains the other.
// For example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6.
// In pairs where one assignment fully contains the other, one Elf in
// the pair would be exclusively cleaning sections their partner will already be cleaning,
//  so these seem like the most in need of reconsideration.
// In this example, there are 2 such pairs.

fn is_in_bounds(bound_1: (i32, i32), bound_2: (i32, i32)) -> bool {
    return bound_1.0 >= bound_2.0 && bound_1.1 <= bound_2.1
}

// Thoughts
// It must be fully contained not partially contained
// containment
fn part_1() {
    let file = std::fs::read_to_string("input")
        .expect("input file should exist");

    let mut foo = 0;
    file.lines().for_each(|line| {
        let mut vec: Vec<_> = vec![];
        line.split(",")
            .for_each(|x| {
                x.split('-')
                    .for_each(|s| {
                        vec.push(s.parse::<i32>().expect("should parse"));
                    })
            });
        // llb - left lower bounds | lub - ... upper bounds | rlb - right ...
        let (llb, lub, rlb, rub): (i32, i32, i32, i32) = (vec[0], vec[1], vec[2], vec[3]);
        if is_in_bounds((llb, lub), (rlb, rub)) || is_in_bounds((rlb, rub), (llb, lub)) {
            foo += 1;
        }
    });
    println!("{}", foo);
}

// It can be partially contained
// overlap
fn part_2() {
    let file = std::fs::read_to_string("input")
        .expect("input file should exist");

    let mut foo = 0;
    file.lines().for_each(|line| {
        let mut vec: Vec<(i32,i32)> = vec![];
        line.split(",")
            .for_each(|x| {
                let (lb, up) = x.split_once('-').expect("should split");
                vec.push((lb.parse::<i32>().expect("should parse"), up.parse::<i32>().expect("should parse")));
            });
        let (x, y) = (vec[0], vec[1]);
        if is_in_bounds((x.0, x.0), y) || is_in_bounds((x.1, x.1), y) || is_in_bounds((y.0, y.0), x) || is_in_bounds((y.1, y.1), x) {
            foo += 1;
        }
    });
    println!("{}", foo);
}

fn main() {
    part_1();
    part_2();
}
