fn main() {
    let input = include_str!("./input2.in");
    let mut input_iter = input.lines();
    let mut sum = 0_u64;
    let mut sum2 = 0_u64;
    loop {
        match input_iter.next() {
            Some(line) => {
                let chars: Vec<_> = line.split(" ")
                                        .collect();
                match chars[1] {
                    "X" => {
                        match chars[0] {
                            "A" => {
                                sum += 3 + 1;
                                sum2 += 0 + 3;
                            },
                            "B" => {
                                sum += 0 + 1;
                                sum2 += 0 + 1;
                            },
                            "C" => {
                                sum += 6 + 1;
                                sum2 += 0 + 2;
                            },
                            _ => panic!("PORQUE MARIA!!!"),
                        }
                    },
                    "Y" => {
                        match chars[0] {
                            "A" => {
                                sum += 6 + 2;
                                sum2 += 3 + 1;
                            },
                            "B" => {
                                sum += 3 + 2;
                                sum2 += 3 + 2;
                            },
                            "C" => {
                                sum += 0 + 2;
                                sum2 += 3 + 3;
                            },
                            _ => panic!("PORQUE MARIA!!!"),
                        }
                    },
                    "Z" => {
                        match chars[0] {
                            "A" => {
                                sum += 0 + 3;
                                sum2 += 6 + 2;
                            },
                            "B" => {
                                sum += 6 + 3;
                                sum2 += 6 + 3;
                            },
                            "C" => {
                                sum += 3 + 3;
                                sum2 += 6 + 1;
                            },
                            _ => panic!("PORQUE MARIA!!!"),
                        }
                    }
                    _ => panic!("PORQUE MARIA!!!"),
                }
            },
            None => break,
        }
    }

    println!();
    println!("day2:\tquest1: {}", sum);
    println!("\tquest2: {}", sum2);
}
