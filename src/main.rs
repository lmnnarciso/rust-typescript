fn main() {
    let data = vec![1, 2, 3];
    let items = data.iter().map(|x| x + 1);
    println!("{:?}", items)
}
