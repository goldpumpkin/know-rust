fn main() {
    let data = vec![12, 24, 1, 4];
    let v = 2;

    // let r = find_pos(data, v);
    // match r {
    //     Some(pos) => println!("Found {} at {}", v, pos),
    //     _ => println!("{} Not Found", v)
    // }

    if let Some(pos) = find_pos(data, v){
        println!("Find {} At {}", v, pos);
    } else {
        println!("{} not find", v);
    }
}

fn find_pos(data: Vec<u32>, v: u32) -> Option<usize> {
    for (pos, item) in data.iter().enumerate() {
        if *item == v {
            return Some(pos);
        }
    }

    None
}