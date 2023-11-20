struct Result {
    highest: i32,
    highest3: i32,
}

fn find_highest(filename: &str) -> Result {
    let mut val: i32 = 0;
    let mut vec = vec![];
    let file = std::fs::read_to_string(filename)
        .unwrap();
    file.lines().for_each(|line| {
        if line != "" {
            let int: i32 = line.parse().unwrap();
            val += int;
        } else {
            vec.push(val);
            val = 0;
        }
    });
    // Push the last item
    vec.push(val);
    vec.sort();
    let item1 = vec.pop().unwrap();
    let item2 = vec.pop().unwrap();
    let item3 = vec.pop().unwrap();
    let res = Result {
        highest: item1,
        highest3: item1 + item2 + item3
    };
    return res;
}

fn main() {
    let res = find_highest("input");
    println!("{}", res.highest);
    println!("{}", res.highest3);
}
