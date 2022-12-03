fn main () {
    let input = include_str!("./input3.in");
    let mut sum = 0_u64;
    for line in input.lines() {
        let (pocket1, pocket2) = line.split_at(line.len() / 2);
        let bytes1 = pocket1.as_bytes();
        let bytes2 = pocket2.as_bytes();
        let mut commons = Vec::<u8>::new();
        for c in bytes1 {
            sum += {
                if bytes2.contains(c) && !commons.contains(c) {
                    let mut tmp: i64 = (*c).into();
                    tmp -= 96;
                    if tmp < 0 {
                        tmp += 58;
                    }
                    commons.push(*c);
                    tmp as u64
                }
                else {
                    0
                }
            }
        }
    }
    println!();
    println!("day3:\tpart 1: {}", sum);

    let mut line_iter = input.lines();
    let mut sum2 = 0_u64;
    loop {
        let buffer = line_iter.next();
        if buffer == None {
            break;
        } else {
            let line1 = buffer.unwrap().as_bytes();

            let line2: Vec<_> = line_iter.next().unwrap()
                                                .as_bytes()
                                                .iter()
                                                .filter(|elm| line1.contains(elm))
                                                .collect();

            let line3: Vec<_> = line_iter.next().unwrap()
                                                .as_bytes()
                                                .iter()
                                                .filter(|elm| line2.contains(elm))
                                                .collect();

            let mut tmp: i64 = (*line3[0]).into();
            tmp -= 96;
            if tmp < 0 {
                tmp += 58;
            }
            sum2 += tmp as u64;
        }
    }
    println!("\tpart 2: {}", sum2);
}
