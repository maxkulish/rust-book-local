fn main() {
    // long v declaration
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(5);

    println!("Vector: {:#?}", v);

    // short
    let mut v = vec![1, 2, 3, 4, 5];
    println!("Vector: {:#?}", v);

    // panicks if index out of range
    let third: &i32 = &v[3];
    println!("The third element is {}", third);

    // get returns None without panicking
    match v.get(5) {
        Some(third) => println!("The fifth element is {}", third),
        None => println!("There is no fifth element"),
    }

    for i in &mut v {
        *i += 1;
        println!("{}", i);
    };

    let word = String::from("नमस्ते!");
    for c in word.bytes() {
        println!("{}", c);
    }
}
