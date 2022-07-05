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
}
