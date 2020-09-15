fn main() {
    
    let die1 = 1;
    let die2 = 5;

    match (die1, die2) {
        (1, 1) => println!("snake eyes!"),
        (5, 5) => println!("you roled two 5s!!!"),
        (5, _) | (_, 5) => {
            println!("you roled at least one 5!");
            println!("move and then roll again!");
        },
        _ => println!("move you piecer!"),
    }
}
