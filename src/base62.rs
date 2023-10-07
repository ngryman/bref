use num::cast::NumCast;
use num::Integer;

/// The base62 alphabet
const ALPHABET: [char; 62] = [
  '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E',
  'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
  'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
  'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
  'y', 'z',
];

/// Convert to base62.
///
/// Add a `to_base62` to any type implementing this trait.
///
/// We could have created a simple function instead but using a trait is a good
/// way to showcase the ergonomics of extending a type.
///
/// For instance, using a trait allows us to chain method calls like this:
/// ```
/// let key = random().to_base62();
/// ```
///
/// Instead of having to nest function calls:
/// ```
/// let key = to_base62(random());
/// ```
///
/// In the end this is just about personal preference :)
pub trait ToBase62 {
  fn to_base62(&self) -> String;
}

/// Convert any integer to base62.
///
/// We are using a combination of traits provided by the `num` crate to make
/// it generic.
///
/// Again, we could have been a bit more straightforward and implement the trait
/// only for `u64` as this is the only type we need. But this is a good
/// opportunity to showcase the power of Rust generics and how you can easily
/// generalize your code.
impl<T: Copy + Integer + NumCast> ToBase62 for T {
  fn to_base62(&self) -> String {
    let len: T = NumCast::from(ALPHABET.len()).unwrap();
    let mut encoded = String::new();
    let mut n = *self;

    // Change `n` from base 10 to base 62
    while n > T::zero() {
      let (quo, rem) = n.div_rem(&len);
      n = quo;
      encoded.insert(0, ALPHABET[rem.to_usize().unwrap()])
    }

    encoded
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn encode_to_base62() {
    assert_eq!(1.to_base62(), "1");
    assert_eq!(2.to_base62(), "2");
    assert_eq!(3.to_base62(), "3");
    assert_eq!(62.to_base62(), "10");
    assert_eq!(63.to_base62(), "11");
    assert_eq!(64.to_base62(), "12");
  }
}
