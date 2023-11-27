use std::collections::HashMap;

fn main() {
    let file = std::fs::read_to_string("input")
        .expect("input file should exist");

    let mut positions: HashMap<i32, Vec<char>> = HashMap::new();

    file.lines().filter(|line| line.contains("[") || line.contains("move")).for_each(|line| {
        if !line.contains("move") {
            line.chars().enumerate().for_each(|(idx, c)| {
                if c != ' ' {
                    if idx == 1 {
                        println!("{}", c);
                        let mut x = positions.get(&1);
                        
                        // let x = match x {
                        //     Some(x) => {
                        //         x.push(c);
                        //         positions.insert(1, x.to_vec());
                        //     }
                        //     None => {
                        //         positions.insert(1, vec![c])
                        //     }
                        // }
                    }
                }
            });
        } else {

        }
    });

    println!("{:?}", positions.get(&1));
}
