
use relog::relog;

#[test]
fn substitution() {
   assert_eq!(relog("a=Int;T<a>"), "T<Int>");
}

#[test]
fn unification() {
   assert_eq!(relog("A<b,C<d>>=A<Int,C<Bool>>;R<b>"), "R<Int>");
}
