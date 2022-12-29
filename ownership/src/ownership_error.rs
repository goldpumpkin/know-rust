fn main() {
    let data = vec![1, 33, 22, 5];
    let data1 = data;
    print!("sum of data1: {}", sum(data1));

    // compile error
    // println!("sum of data1: {}", sum(data1));
    // println!("sum of data:{}", sum(data));
}

fn sum(data: Vec<u32>) -> u32 {
    data.iter().fold(0, |acc, x| acc + x)
}