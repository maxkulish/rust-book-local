#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// fn main() {}

#[cfg(test)]
mod tests {
    use super::*; // bring the code under test in the outer module into the scope

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test2() {
        assert_eq!(6 + 6, 12);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[ignore]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 != 4"))
        }
    }
}

// cargo test -- is separator
// cargo test -- --help                displays the options you can use after the separator --
// cargo test -- --test-threads=1      run tests one by one
// cargo test -- --show-output         if we want to see printed values
// cargo test exploration              runs only one test function 'exploration'
// cargo test -- --ignored             runs only the ignored tests
