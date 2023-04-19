fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("the filename to be passed in");

    let file = std::fs::read_to_string(arg).expect("unable to read the file to string");
    let foo = Some(5);

    file.lines().for_each(|line| {
        if let Ok(x) = line.parse::<usize>() {
            println!("{}", x)
        } else {
            println!("Line not a number")
        }
    })
}
