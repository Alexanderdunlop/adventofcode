fn main() {
    let file = std::fs::read_to_string("input")
        .unwrap();
    let mut bar = 0;
    // replace with number in between for numbers like `twone`
    file
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
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
                    x = foo;
                    i = 0;
                }
            }
            while y == 0 {
                let foo = chars[chars.len() - i - 1] as i32 - 0x30;
                if foo > 9 {
                    i = i + 1;
                } else {
                    y = foo;
                }
            }
            bar += x * 10 + y;
        });
    
    println!("{}", bar);
}
