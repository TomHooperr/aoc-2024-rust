
pub fn run(input: &str) {
    let data: Vec<Vec<i32>> = input.lines()
        .map(|line| line.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect())
        .collect();

    // Part 1
    let safe_reports: Vec<Vec<i32>> = data.iter()
        .filter(|report|
            report.is_sorted_by(|a, b| a < b && a.abs_diff(*b) <= 3)
                || report.is_sorted_by(|a, b| a > b && a.abs_diff(*b) <= 3))
        .map(|report| report.clone()).collect();

    let part1_res = safe_reports.len();

    println!("Part 1: {}", part1_res);

    // Part 2
    let damp_reports: Vec<Vec<i32>> = data.iter().filter(|report| {
        //let safe = is_safe(report);
        let safe = is_safe(report);
        
        // if safe {
        //     println!("{:?}: {}", report, safe);
        // }
        
        safe
    }).map(|report| report.to_vec()).collect();

    let part2_res = damp_reports.len();

    println!("Part 2: {}", part2_res);
}

fn is_safe(report: &Vec<i32>) -> bool {
    let asc = |rep: &Vec<i32>| 
        rep.windows(2)
            .all(|w| w[0] < w[1] 
                && (1..=3).contains(&w[0].abs_diff(w[1])));

    let dsc = |rep: &Vec<i32>|
        rep.windows(2)
            .all(|w| w[0] > w[1] 
                && (1..=3).contains(&w[0].abs_diff(w[1])));
    
    return asc(report) || dsc(report) || (0..report.len()).any(|i| {
        let mut report = report.clone();
        report.remove(i);
        asc(&report) || dsc(&report)
    });
}

// fn is_safe(report: &Vec<i32>) -> bool {
//     is_safe_impl(report, report, false)
// }
// 
// fn is_safe_impl(orig: &Vec<i32>, curr: &Vec<i32>, stop: bool) -> bool {
//     let indexes: Vec<usize> = (0..curr.len()).collect();
//     let mut asc: Option<bool> = None;
//     let mut safe = true;
// 
//     for window in indexes.windows(2) {
//         let a = curr[window[0]];
//         let b = curr[window[1]];
// 
//         if asc == None && a != b {
//             asc = Some(a < b);
//         }
// 
//         if !(1..=3).contains(&a.abs_diff(b)) || asc == None || asc != Some(a < b) {
//             if stop {
//                 return false;
//             }
//             
//             let mut reduced = orig.clone();
//             
//             if (window[0] > 0) {
//                 reduced.remove(window[0]-1);
// 
//                 if is_safe_impl(orig, &reduced, true) {
//                     break;
//                 }
//             }
// 
//             reduced = orig.clone();
//             reduced.remove(window[0]);
// 
//             if is_safe_impl(orig, &reduced, true) {
//                 break;
//             }
// 
//             reduced = orig.clone();
//             reduced.remove(window[1]);
// 
//             if is_safe_impl(orig, &reduced, true) {
//                 break;
//             }
//             
//             safe = false;
//             break;
//         }
//     }
// 
//     safe
// }