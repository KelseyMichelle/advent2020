use std::fs;
use std::convert::TryInto;

pub fn advent9() {
    const FILE_NAME: &str = "input9";

    println!("[ Advent day 1 ]");
    let contents = fs::read_to_string(FILE_NAME)
        .expect("failed to read file");
    let contents = contents.split("\n");
    let contents = contents.collect::<Vec<&str>>();
    let preamble: usize = 25;
    for x in preamble..contents.len() {
        let mut valid = false;
        let x_match: usize = match contents[x].trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    continue;
                },
        };
        for y in 1..(preamble+1) {
            let y: usize = (x-y).try_into().unwrap();
            let y: usize = match contents[y].trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    continue;
                },
             };
            for z in 1..(preamble+1) {
                let z: usize = (x-z).try_into().unwrap();
                let z: usize = match contents[z].trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        continue;
                    },
                };
                if z != y && y + z == x_match {
                    valid = true;
                }
            }
        }
        if !valid {
            println!("invalid number: {}", x_match);
            for y in 0..(contents.len()) {
                let mut therange = vec![];
                let mut curr_index = y;
                let mut total = 0;
                while total < x_match {
                    let y: usize = match contents[curr_index].trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            continue;
                        },
                    };
                    therange.extend(vec![&y]);
                    total+=y;
                    curr_index += 1;
                }
                if total == x_match && therange.len() != 1 {
                    let mut min = 26134589;
                    let mut max = 0;
                    for v in therange {
                        if v < min {
                            min = v;
                        }
                        if v > max {
                            max = v;
                        }
                    }
                    println!("min: {}\nmax:{}", min, max);
                    println!("weakness: {}", min+max);
                }  
            }
        }
    }
}
