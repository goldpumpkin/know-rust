fn main() {

    let arr = ['h', 'e', 'l', 'l', 'o'];
    let vec = vec!['h', 'e', 'l', 'l', 'o'];

    let s1 = &arr[..2];
    let s2 = &vec[..2];

    let s = String::from("hello");
    let s3 = &s[..2];

    println!("s1: {:?}, s2: {:?}, s3: {:?}", s1, s2, s3);

    // &[char] 是否相等取决于长度和内容是否相等
    assert_eq!(s1, s2);
    assert_eq!(s2, s3.chars().collect::<Vec<_>>());
    assert_eq!(String::from_iter(s2), s3);
}
