use testing::{sploosh, splish};

#[cfg(test)]
mod more_tests {
   use super::*;

   #[test]
   fn test_sploosh_splish() {
      assert_eq!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)), 4);
   }
}