use std::collections::HashMap;

fn main() {
    let file = std::fs::read_to_string("input")
        .expect("input file should exist");

    let mut positions: HashMap<i32, Vec<char>> = HashMap::new();

    // || line.contains("move")
    file.lines().enumerate().filter(|(idx, k_line)| {
        if *idx == 8 {
            k_line.chars().filter(|c| *c != ' ').for_each(|c| {
                // println!("{}", c)
                // TODO: create a mapping vec, so the below chars know where to be inserting in the positions table.
                // Need to learn more about the borrow check first, as positions in this case can not be borrowed and then read.
                &positions.insert(c as i32, vec![]);
            })
        }
        return k_line.contains("[")
    }).for_each(|(_, line)| {
        println!("{}", line);
        line.chars().enumerate().for_each(|(idx, c)| {
            if c != ' ' {
                if idx == 1 {
                    println!("{}", c);
                    let x = positions.get(&1);
                    
                    let _ = match x {
                        Some(y) => {
                            let mut z = y.clone();
                            z.push(c);
                            // positions.insert(1, z.to_vec());
                        }
                        None => {
                            let mut v = vec![];
                            v.push(c);
                            // positions.insert(1, v);
                        }
                    };
                }
            }
        });
    });

    println!("{:?}", positions.get(&1));
}
