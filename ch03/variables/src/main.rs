
fn main() {
    let x = plus_one(5);

    println!("x: {}", x);
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}