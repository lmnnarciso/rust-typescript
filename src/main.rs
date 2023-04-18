fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("the filename to be passed in");

    let file = std::fs::read_to_string(arg).expect("unable to read the file to string");

    file.lines().for_each(|line| println!("{}", line))
}
