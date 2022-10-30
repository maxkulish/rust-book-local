fn main() {

    let mut s = "initial contents".to_string();
    s.push_str("bar");

    println!("string: {}", s);

    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("string after add: {}", s3);

}
