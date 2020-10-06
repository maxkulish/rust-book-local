
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let num_list = vec![12, 52, 10, 59];

    let result = largest(&num_list);
    println!("The largest number is {}", result);

    let char_list = vec!["y", "m", "a", "q"];
    let result = largest(&char_list);
    println!("The largest number is {}", result);
}

