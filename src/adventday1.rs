use std::fs;

pub fn advent1() {
    const FILE_NAME: &str = "input1";

    println!("[ Advent day 1 ]");
    let contents = fs::read_to_string(FILE_NAME)
        .expect("failed to read file");
    let contents = contents.split("\n");
    let mut evens = Vec::new();
    let mut odds = Vec::new();
    let mut pt1_ans: u64 = 0;
    let mut pt2_ans: u64 = 0;

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
    let mut found1 = false;
    let mut found2 = false;
    for x in evens.iter() {
        for y in evens.iter() {
            for z in evens.iter() {
                if found2 {break;}
                if x + y + z == 2020 {
                    let a: u64 = x*y*z;
                    pt2_ans = a;
                    found2 = true;
                    break;
                }
            }
            if x + y == 2020 {
                    let a: u64 = x*y;
                    pt1_ans = a;
                    found1 = true;
                    break;
                }
            
        }
        if found2 && found1 {break;}
    }
    
    for x in odds.iter() {
        for y in odds.iter() {
            for z in odds.iter() {
                if found2 {break;};
                if x + y + z == 2020 {
                    let a: u64 = x*y*z;
                    pt2_ans = a;
                    found2 = true;
                    break;
                }
            }
            if found1 {break;};
            if x + y == 2020 {
                    let a: u64 = x*y;
                    pt1_ans = a;
                    found1 = true;
                    break;
                }
        }
        if found2 && found1 {break;}
    }
    println!("part 1 solution: {}", pt1_ans);
    println!("part 2 solution: {}", pt2_ans);
    println!("");

}
