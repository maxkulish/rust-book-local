//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given
/// 
/// # Examples
/// ```
/// let arg = 5;
/// let answer = add_one::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one() {
        let x = 2;
        assert_eq!(x + 1, add_one(x));
    }
}
