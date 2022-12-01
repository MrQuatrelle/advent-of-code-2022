fn main() {
    let input = include_str!("./input1.in");
    let mut iter = input.lines();
    let mut curr_sum = 0_u32;
    let mut sums = Vec::<u32>::new();
    loop {
        match iter.next() {
            Some("") => {
                sums.push(curr_sum.clone());
                curr_sum = 0;
            },
            Some(calorie) => {curr_sum += calorie.parse::<u32>().expect("PORQUE MARIA!!!");},
            None => break, 
        }
    }
    sums.sort_by(|a, b| b.cmp(a));
    println!();
    println!("day1:\tpart 1: {}", sums[0]);
    println!("\tpart 2: {}", sums.iter().take(3).sum::<u32>());
}
