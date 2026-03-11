fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(10);
    v.push(11);
    println!("{:?}", v);

    v = vec![4, 5, 6];
    println!("{:?}", v);

    v = vec![10; 5];
    println!("{:?}", v);

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
}