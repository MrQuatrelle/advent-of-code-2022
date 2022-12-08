fn main() {
    let input = include_str!("./input6.in");
    let mut marker: Vec<char> = Vec::new();
    for (index, c) in input.chars().enumerate() {
        // for part 1
        // if marker.len() > 3 {
        //     marker.pop();
        // }
        if marker.len() > 13 {
            marker.pop();
        }
        marker.insert(0, c);
        let mut buffer = marker.clone();
        buffer.sort();
        buffer.dedup();
        // for part 1
        // if buffer.len() == 4 {
        //     println!("day6\tpart1: {}",index + 1);
        //     break;
        // }
        if buffer.len() == 14 {
            println!("day6\tpart2: {}",index + 1);
            break;
        }
    }
}
