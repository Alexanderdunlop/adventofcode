fn main() {
    let file = std::fs::read_to_string("input")
        .unwrap();
    let mut bar = 0;
    // replace with number in between for numbers like `twone`
    file
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "4")
        .replace("five", "5e")
        .replace("six", "6")
        .replace("seven", "7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
        .lines()
        .for_each(|line| {
            let chars: Vec<_> = line.chars().collect();
            let mut x = 0;
            let mut y = 0;
            let mut i = 0;
    
            while x == 0 {
                let foo = chars[i] as i32 - 0x30;
                if foo > 9 {
                    i = i + 1;
                } else {
                    x = foo * 10;
                    i = 0;
                }
            }
            while y == 0 {
                let foo = chars[chars.len() - i - 1] as i32 - 0x30;
                if foo > 9 {
                    i = i + 1;
                } else {
                    y = foo;
                    i = 0;
                }
            }

            bar += x + y;
        });
    
    println!("{}", bar);
}
