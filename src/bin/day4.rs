fn main() {
    let input = include_str!("./input4.in");
    let mut counter = 0_u32;
    let mut counter2 = 0_u32;
    for line in input.lines() {
        let bounds: Vec<_> = line.split(",").collect();
        let mut bound_iter = bounds[0].split("-");
        let lowerbound1 = bound_iter.next().unwrap().parse::<u32>().expect("bad input");
        let upperbound1 = bound_iter.next().unwrap().parse::<u32>().expect("bad input");
        let mut bound_iter = bounds[1].split("-");
        let lowerbound2 = bound_iter.next().unwrap().parse::<u32>().expect("bad input");
        let upperbound2 = bound_iter.next().unwrap().parse::<u32>().expect("bad input");
        if (lowerbound1 >= lowerbound2 && upperbound1 <= upperbound2) ||
            (lowerbound1 <= lowerbound2 && upperbound1 >= upperbound2) {
                counter += 1;
        }
        if (lowerbound1 <= lowerbound2 && lowerbound2 <= upperbound1) ||
            (lowerbound1 <= upperbound2 && upperbound2 <= upperbound1) ||
            (lowerbound1 >= lowerbound2 && upperbound1 <= upperbound2) ||
            (lowerbound1 <= lowerbound2 && upperbound1 >= upperbound2) {
            counter2 += 1;
        }
    }
    println!("day4:\tpart1:{}", counter);
    println!("\tpart2:{}", counter2);
}
