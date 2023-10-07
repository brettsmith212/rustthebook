// ** Generics ** //
// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// fn main() {
//     let numbers_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&numbers_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

// ** Traits ** //
use aggregator::{Summary, Tweet};
fn main() {
    let tweet = Tweet {
        username: String::from("Glen"),
        content: String::from("yo whaddup dog this is your boy gl3n!"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
