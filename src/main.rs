struct Custom {
    age: usize,
    name: String,
}

enum Item {
    Number(usize),
    String(String),
    MyCustom(Custom),
}

fn append(items: &mut Vec<Item>) {
    items.push(Item::String("hello, Fem".to_string()));
}

fn multiply(num: Option<usize>) -> Option<usize> {
    return Some(num? * 5);
}

fn main() {
    let foo = multiply(None);

    println!("{:?}", foo)
}
