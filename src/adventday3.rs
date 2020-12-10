use std::fs;

pub fn advent3() {
    const FILE_NAME: &str = "input3";
    println!("[ Advent day 3 ]");
    let contents = fs::read_to_string(FILE_NAME)
        .expect("failed to read file");
    let contents = contents.split("\n");

    let content_vector = contents.collect::<Vec<&str>>();
    let num_lines = content_vector.len();
    // part 1
    // let mut pt1_count = 0;
    // let mut x = 0;
    // let mut ycnt = 0;
    // let x_offset = 3;
    // print!("[");
    // for line in contents {
    //     let point = x % line.len();
    //     let line = line.split("");
    //     let line = line.collect::<Vec<&str>>();
    //     if line[point+1] == "#" {
    //         print!("{},", ycnt);
    //         pt1_count += 1;      
    //     }
    //     ycnt+=1;
    //     x += x_offset;
    // }
    // print!("]");
    // println!("part 1 tree count: {}", pt1_count);

    let offsets = [[1,1], [3,1], [5,1], [7,1], [1,2]];
    let mut pt2_ans: u128 = 1;
    for offset in offsets.iter() {
        let mut count = 0;
        let mut column = 0;
        let mut row = 0;
        while row < num_lines {
            let line = content_vector[row];
            let point = column % line.len();
            let line = content_vector[row].split("");
            let line = line.collect::<Vec<&str>>();
            if line[point+1] == "#"
            {
                count+=1;
            }
            column += offset[0];
            row+=offset[1];
            
        }
        println!("[ {}, {} ], [{}, {}]: {}", offset[0], offset[1],column, row, count);
     
        pt2_ans*=count;
    }

    
    println!("part 2 tree multiples: {}", pt2_ans);
}