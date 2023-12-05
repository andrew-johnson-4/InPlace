
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
   assert_eq!(parse_relog_term("A<Int,C<Bool>>").to_string(), "A<Int,C<Bool>>");
}

#[test]
fn parse_prog() {
   assert_eq!(parse_relog_prog("a").to_string(), "a");
   assert_eq!(parse_relog_prog("ab").to_string(), "ab");
   assert_eq!(parse_relog_prog("A").to_string(), "A");
   assert_eq!(parse_relog_prog("Ab").to_string(), "Ab");
   assert_eq!(parse_relog_prog("A<b>").to_string(), "A<b>");
   assert_eq!(parse_relog_prog("A<b,C>").to_string(), "A<b,C>");
   assert_eq!(parse_relog_prog("A<b,,>").to_string(), "A<b,!,!>");

   assert_eq!(parse_relog_prog("x=y;a").to_string(), "x=y;a");
   assert_eq!(parse_relog_prog("x=y;ab").to_string(), "x=y;ab");
   assert_eq!(parse_relog_prog("x=y;A").to_string(), "x=y;A");
   assert_eq!(parse_relog_prog("x=y;Ab").to_string(), "x=y;Ab");
   assert_eq!(parse_relog_prog("x=y;A<b>").to_string(), "x=y;A<b>");
   assert_eq!(parse_relog_prog("x=y;A<b,C>").to_string(), "x=y;A<b,C>");
   assert_eq!(parse_relog_prog("x=y;A<b,,>").to_string(), "x=y;A<b,!,!>");
   assert_eq!(parse_relog_prog("").to_string(), "!");
   assert_eq!(parse_relog_prog(";").to_string(), "!");
   assert_eq!(parse_relog_prog("x;").to_string(), "!");
}

#[test]
fn substitution() {
   assert_eq!(relog("a=Int;T<a>"), "T<Int>");
}

#[test]
fn unification() {
   assert_eq!(relog("A<b,C<d>>=A<Int,C<Bool>>;R<b>"), "R<Int>");
}
