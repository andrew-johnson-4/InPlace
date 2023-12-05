
use relog::*;

#[test]
fn parse_term() {
   assert_eq!(parse_relog_term("a").to_string(), "a");
   assert_eq!(parse_relog_term("ab").to_string(), "ab");
   assert_eq!(parse_relog_term("A").to_string(), "A");
   assert_eq!(parse_relog_term("Ab").to_string(), "Ab");
   assert_eq!(parse_relog_term("A<b>").to_string(), "A<b>");
   assert_eq!(parse_relog_term("A<b,C>").to_string(), "A<b,C>");
   assert_eq!(parse_relog_term("A<b,,>").to_string(), "A<b,!,!>");
}

#[test]
fn substitution() {
   assert_eq!(relog("a=Int;T<a>"), "T<Int>");
}

#[test]
fn unification() {
   assert_eq!(relog("A<b,C<d>>=A<Int,C<Bool>>;R<b>"), "R<Int>");
}
