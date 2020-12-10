use std::fs;
use std::convert::TryInto;

pub fn advent2() {
    const FILE_NAME: &str = "input2";

    println!("file: {}", FILE_NAME);
    let contents = fs::read_to_string(FILE_NAME)
        .expect("failed to read file");
    let contents = contents.split("\n");
    let mut numcomplypt1 = 0;
    let mut numcomplypt2 = 0;
    for p in contents {
        let b = p.trim().split(" ");
        let b = b.collect::<Vec<&str>>();
        
        let curr_letter = b[1].get(0..1).unwrap();
        let mm = b[0].split("-");
        let mm = mm.collect::<Vec<&str>>();
        let min = mm[0];
        let max = mm[1];
        let min: u64 = match min.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            },
        };
        let max: u64 = match max.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            },
        };
        let letters = b[2].split("");

        // [part 2]
        let min: usize = (min).try_into().unwrap();
        let max: usize = (max).try_into().unwrap();
        let letters = letters.collect::<Vec<&str>>();
        let minletter = letters[min];
        let maxletter = letters[max];
        if (minletter == curr_letter) ^ (maxletter == curr_letter) {
            numcomplypt2 += 1;
        }

        // [part 1]

        let mut c = 0;
        for letter in letters {
            if letter == curr_letter {c+=1;};
            
        }
        
        if c >= min && c <= max {
            numcomplypt1 += 1;
        }

    }
    println!("part 1 compliant passwords: {}", numcomplypt1);
    println!("part 2 compliant passwords: {}", numcomplypt2);

}
