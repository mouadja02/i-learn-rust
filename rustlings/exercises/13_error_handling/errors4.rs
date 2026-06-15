#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // TODO: This function shouldn't always return an `Ok`.
        // Read the tests below to clarify what should be returned.
        match value {
            v if v > 0 => Ok(PositiveNonzeroInteger(v as u64)),
            0 => Err(CreationError::Zero),
            v if v < 0 => Err(CreationError::Negative),
            _ => Err(CreationError::Negative),
        }
    }
}

fn main() {
    let positive_integer : i64 = 10;
    let _ = PositiveNonzeroInteger::new(positive_integer);
    println!("Success! Created a PositiveNonzeroInteger from {positive_integer}");

    let negative_case = PositiveNonzeroInteger::new(-10);
    println!("We expected an error but got: {negative_case:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
