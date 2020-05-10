extern crate proptest;
extern crate piston_obj;

use proptest::prelude::*;
use piston_obj::obj;

proptest! {
  #[test]
  fn detect_corrupted_data(s in "[:alphanum:]+") {
    let result = obj::parse(s);
    assert!(result.is_err());
  }
}
