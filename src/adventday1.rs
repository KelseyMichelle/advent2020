use std::fs;

pub fn advent1() {
    const FILE_NAME: &str = "input1";

    println!("file: {}", FILE_NAME);
    let contents = fs::read_to_string(FILE_NAME)
        .expect("failed to read file");
    let contents = contents.split("\n");
    let mut evens = Vec::new();
    let mut odds = Vec::new();

    for x in contents {
        let x: u64 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            },
        };
        if x%2 == 0 {
            evens.push(x);
            continue;
        } else {
            odds.push(x);
        }

    }
    for x in evens.iter() {
        for y in evens.iter() {
            for z in evens.iter() {
                if x + y + z == 2020 {
                    let a: u64 = x*y*z;
                    println!("ans: {}", a);
                }
            }
                
        }
    }
    for x in odds.iter() {
        for y in odds.iter() {
            for z in odds.iter() {
                if x + y + z == 2020 {
                    let a: u64 = x*y*z;
                    println!("ans: {}", a);
                }
            }
            
        }
    }

}
