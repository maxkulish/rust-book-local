#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    
    let home = IpAddr::V4(127, 0, 0, 255);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home: {:#?}", home);
    println!("Loopback: {:#?}", loopback);
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }


}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
