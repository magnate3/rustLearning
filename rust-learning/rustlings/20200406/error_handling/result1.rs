// result1.rs
// Make this test pass! Execute `rustlings hint result1` for hints :)

// Here is that number > = < is not All. using _.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Ok(PositiveNonzeroInteger(value as u64))
        match value {
               value if value>0 => return Ok(PositiveNonzeroInteger(value as u64)),
                0 =>  return Err(CreationError::Zero),
                _  => return Err(CreationError::Negative),
                } 
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
