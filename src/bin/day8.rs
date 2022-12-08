struct Tree {
    height: i32,
    seen: bool,
}

fn check_visibility(map: &mut Vec<Vec<Tree>>) {
    for row in map.into_iter() {
        let mut max = -1_i32;
        
        for mut tree in row.into_iter() {
            if max < tree.height {
                max = tree.height;
                tree.seen = true;
            }
        }
        max = -1_i32;
        for mut tree in row.into_iter().rev() {
            if max < tree.height {
                max = tree.height;
                tree.seen = true;
            }
        }
    }
    for i in 0..map[0].len() {
        let mut max = -1_i32;
        for row in map.into_iter() {
            if max < row[i].height {
                max = row[i].height;
                row[i].seen = true;
            }
        }
        max = -1_i32;
        for row in map.into_iter().rev() {
            if max < row[i].height {
                max = row[i].height;
                row[i].seen = true;
            }
        }
    }
}

fn main() {
    let input = include_str!("./input8.in");
    let mut map = Vec::<Vec<Tree>>::new();
    
    for s in input.lines() {
        let c_iter = s.chars();
        let mut row = Vec::<Tree>::new();
        for c in c_iter {
            row.push(Tree{
                height: c.to_digit(10).unwrap() as i32,
                seen: false});
        } 
        map.push(row);
    }

    check_visibility(&mut map);

    let mut counter = 0_u64;
    for row in map {
        for t in row {
            if t.seen == true {
                counter += 1;
            }
        }
    }
    println!("day8\tpart1: {}", counter);
}
