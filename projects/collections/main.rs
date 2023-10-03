use std::collections::HashMap;

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let v = vec![1, 2, 3];
    let mut w = Vec::new();
    w.push(5);
    w.push(6);
    w.push(7);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("3rd element is {}", third),
        None => println!("There is no third element"),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("row: {:?}", row);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
