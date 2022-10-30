fn main() {
    let x = five();

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is {}", x);
    }

    println!("The value of x is {}", x);

    let y: f32 = 3.0;

    println!("The value of y is {}", y);

    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;

    let remainder = 43 % 5;

    println!("The value of quotient is {}", quotient);
    println!("The value of floored is {}", floored);
    println!("The value of remainder is {}", remainder);

    let tup: (i32, f64, u8) = (500, 6.3, 1);

    println!("The value of tup is {}, {}", tup.0, tup.1);

    let a = [1, 2, 3, 4, 5];
    println!("The value of a is {}", a[0]);

    another_function(x);

    let number = 7;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is {}", number);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    for number in (1..4).rev() {
        println!("{}!", number);
    }

}
    

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}