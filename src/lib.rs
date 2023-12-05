
pub enum RelogTerm {
   Atomic(String),
   Var(String),
   Compound(String,Vec<RelogTerm>),
}

pub struct RelogProg {
   bindings: Vec<(RelogTerm,RelogTerm)>,
   returns: RelogTerm,
}

pub fn parse_relog(s: &str) -> RelogTerm {
   RelogTerm::Atomic("".to_string())
}

pub fn relog(s: &str) -> String {
   /*
   source: https://kti.mff.cuni.cz/%7Ebartak/prolog/data_struct.html
   unify(A,B):-
      atomic(A),atomic(B),A=B.
   unify(A,B):-
      var(A),A=B.            % without occurs check
   unify(A,B):-
      nonvar(A),var(B),A=B.  % without occurs check
   unify(A,B):-
      compound(A),compound(B),
      A=..[F|ArgsA],B=..[F|ArgsB],
      unify_args(ArgsA,ArgsB)

   unify_args([A|TA],[B|TB]):-
      unify(A,B),
      unify_args(TA,TB).
   unify_args([],[])
   */
   s.to_string()
}
