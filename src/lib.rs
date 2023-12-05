
use std::collections::HashMap;

#[derive(PartialEq,Eq,Clone)]
pub enum RelogTerm {
   Reject,
   Atomic(String),
   Var(String),
   Compound(String,Vec<RelogTerm>),
}
impl RelogTerm {
   pub fn to_string(&self) -> String {
      match self {
         RelogTerm::Reject => format!("!"),
         RelogTerm::Atomic(a) => format!("{}", a),
         RelogTerm::Var(v) => format!("{}", v),
         RelogTerm::Compound(x,xs) => format!("{}<{}>", x, xs.iter().map(|x|x.to_string()).collect::<Vec<String>>().join(",") ),
      }
   }
}

pub struct RelogProg {
   bindings: Vec<(RelogTerm,RelogTerm)>,
   returns: RelogTerm,
}
impl RelogProg {
   pub fn to_string(&self) -> String {
      let mut s = String::new();
      for (l,r) in self.bindings.iter() {
         s += &format!("{}={};",l.to_string(),r.to_string());
      }
      s += &self.returns.to_string();
      s
   }
}

pub fn parse_relog_term(s: &str) -> RelogTerm {
   let s = s.as_bytes();
   if s.len()==0 { RelogTerm::Reject }
   else if "<>,;=".contains(s[0] as char) { RelogTerm::Reject }
   else if s[0].is_ascii_lowercase() { RelogTerm::Var(std::str::from_utf8(s).unwrap().to_string()) }
   else if s[0].is_ascii_uppercase() {
      if s[s.len()-1] == b'>' {
         if let Some((head,tail)) = std::str::from_utf8( &s[..s.len()-1] ).unwrap().to_string().split_once('<') {
            let mut tail_terms = Vec::new();
            let mut depth = 0;
            let mut run = String::new();
            for c in tail.bytes() {
               if c==b'<' { depth += 1; }
               else if c==b'>' { depth -= 1; }
               else if depth==0 && c==b',' {
                  tail_terms.push(parse_relog_term(&run));
                  run = String::new();
               } else { run.push(c as char); }
            }
            tail_terms.push(parse_relog_term(&run));
            RelogTerm::Compound(head.to_string(),tail_terms)
         } else { RelogTerm::Reject }
      } else {
         RelogTerm::Atomic(std::str::from_utf8(s).unwrap().to_string())
      }
   }
   else { RelogTerm::Reject }
}

pub fn parse_relog_prog(s: &str) -> RelogProg {
   let mut s = s.split(";").collect::<Vec<&str>>();
   let ret = parse_relog_term(s.pop().unwrap()); //there should always be one string, even if it is empty
   let mut bindings = Vec::new();
   for b in s {
      if let Some((l,r)) = b.split_once("=") {
         bindings.push( (parse_relog_term(l), parse_relog_term(r)) );
      }
   }
   RelogProg {
      bindings: bindings,
      returns: ret
   }
}

pub fn relog_unify(ctx: &mut HashMap<String,RelogTerm>, l: RelogTerm, r: RelogTerm) -> RelogTerm {
   match (l,r) {
      (RelogTerm::Atomic(l),RelogTerm::Atomic(r)) if l==r => { RelogTerm::Atomic(l.clone()) },
      (RelogTerm::Var(l),r) => {
         ctx.insert(l, r.clone());
         r.clone()
      },
      _ => RelogTerm::Reject,
   }
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
}
pub fn relog_reify(ctx: &HashMap<String,RelogTerm>, x: RelogTerm) -> RelogTerm {
   match x {
      RelogTerm::Reject => { RelogTerm::Reject },
      RelogTerm::Atomic(x) => { RelogTerm::Atomic(x.clone()) },
      RelogTerm::Var(x) => {
         if let Some(r) = ctx.get(&x) { r.clone() }
         else { RelogTerm::Var(x.clone()) }
      },
      RelogTerm::Compound(x,xs) => {
         RelogTerm::Compound( x.clone(), xs.into_iter().map(|x| relog_reify(ctx,x)).collect::<Vec<RelogTerm>>() )
      }
   }
}

pub fn relog(s: &str) -> String {
   let mut ctx: HashMap<String,RelogTerm> = HashMap::new();
   let p = parse_relog_prog(s);
   for (l,r) in p.bindings {
      if relog_unify(&mut ctx, l, r) == RelogTerm::Reject {
         return RelogTerm::Reject.to_string();
      }
   }
   relog_reify(&ctx, p.returns).to_string()
}
