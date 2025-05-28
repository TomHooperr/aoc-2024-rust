use regex::Regex;

pub fn run(input: &str) {
    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let sum_multiples = |start, end| re_mul.captures_iter(&input[start..end]).map(|caps| {
        let a: i32 = caps[1].parse().unwrap();
        let b: i32 = caps[2].parse().unwrap();
        (a, b)
    }).fold(0, |sum, mult| sum + (mult.0 * mult.1));
    
    println!("{}", sum_multiples(0, input.len() - 1));
    
    let mut sum_2 = 0;
    let mut start = 0;
    let mut end = 0;
    let re_ins = Regex::new(r"(do|don't)\(\)").unwrap();
    
    let mut is_do = true;
    
    for cap in re_ins.captures_iter(&input) {
        //println!("{}", &cap[1]);
        
        end = cap.get(0).unwrap().start();
        
        if is_do {
            sum_2 += sum_multiples(start, end);
        }
        
        is_do = match &cap[1] {
            "do" => true,
            "don't" => false,
            _ => true,
        };
        
        start = cap.get(0).unwrap().end();
    }

    if is_do {
        sum_2 += sum_multiples(start, input.len() - 1);
    }
    
    println!("{}", sum_2);
}