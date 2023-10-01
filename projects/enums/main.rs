enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    println!("enums");

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    let x = 1;
    let y = Some(5);

    let y = match y {
        Some(n) => n,
        None => panic!("Not valid"),
    };

    println!("Sum of {x} + {y} = {}", x + y);
}
