use std::collections::HashMap;

pub fn run(input: &str) {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    input.lines().for_each(|str| {
        let row: Vec<&str> = str.split_whitespace().collect();
        if let Some(left_val) = row.get(0) {
            left.extend(left_val.parse::<u32>().ok());
        }
        if let Some(right_val) = row.get(1) {
            right.extend(right_val.parse::<u32>().ok());
        }
        //println!("{}, {}", row.get(0).unwrap(), row.get(1).unwrap())
    });

    left.sort();
    right.sort();

    let mut part1_res = 0;
    for (left_val, right_val) in left.iter().zip(right.iter()) {
        part1_res += left_val.abs_diff(*right_val);
    }

    //println!("{:?}", left);
    println!("Part 1: {}", part1_res);

    let mut part2_res = 0;
    
    let mut count_map: HashMap<u32, u32> = HashMap::new();
    for &val in &right {
        *count_map.entry(val).or_insert(0) += 1;
    }

    for val in &left {
        if let Some(count) = count_map.get(val) {
            part2_res += val * count;
        }
    }

    println!("Part 2: {}", part2_res);
}