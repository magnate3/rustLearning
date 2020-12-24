// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

// I AM DONE It can be again.
pub fn is_ok(c:char)->Option<u32>{
	
	c.to_digit(10)
	
}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(is_ok('2'),Some(2));
    }
}
