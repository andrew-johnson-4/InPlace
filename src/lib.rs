

use std::collections::HashMap;

#[derive(PartialEq,Eq,Clone,Hash)]
enum RelogTerm {
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

struct RelogProg {
   bindings: Vec<(RelogTerm,RelogTerm)>,
   unifications: Vec<(RelogTerm,RelogTerm)>,
   returns: RelogTerm,
}
#[cfg(test)]
impl RelogProg {
   pub fn to_string(&self) -> String {
      let mut s = String::new();
      for (l,r) in self.bindings.iter() {
         s += &format!("{}:={};",l.to_string(),r.to_string());
      }
      for (l,r) in self.unifications.iter() {
         s += &format!("{}={};",l.to_string(),r.to_string());
      }
      s += &self.returns.to_string();
      s
   }
}

fn parse_relog_term(s: &str) -> RelogTerm {
   let s = s.as_bytes();
   if s.len()==0 { RelogTerm::Reject }
   else if "<>,:;=".contains(s[0] as char) { RelogTerm::Reject }
   else if s[0].is_ascii_uppercase() {
      if s[s.len()-1] == b'>' {
         if let Some((head,tail)) = std::str::from_utf8( &s[..s.len()-1] ).unwrap().to_string().split_once('<') {
            let mut tail_terms = Vec::new();
            let mut depth = 0;
            let mut run = String::new();
            for c in tail.bytes() {
               if c==b'<' { depth += 1; run.push(c as char); }
               else if c==b'>' { depth -= 1; run.push(c as char); }
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
   else { RelogTerm::Var(std::str::from_utf8(s).unwrap().to_string()) }
}

fn parse_relog_prog(s: &str) -> RelogProg {
   let mut s = s.split(";").collect::<Vec<&str>>();
   let ret = parse_relog_term(s.pop().unwrap()); //there should always be one string, even if it is empty
   let mut bindings = Vec::new();
   let mut unifications = Vec::new();
   for b in s {
      if let Some((l,r)) = b.split_once(":=") {
         bindings.push( (parse_relog_term(l), parse_relog_term(r)) );
      } else if let Some((l,r)) = b.split_once("=") {
         unifications.push( (parse_relog_term(l), parse_relog_term(r)) );
      }
   }
   RelogProg {
      bindings: bindings,
      unifications: unifications,
      returns: ret
   }
}

fn relog_apply(ctx: &mut HashMap<RelogTerm,RelogTerm>, x: RelogTerm) -> RelogTerm {
   for (k,v) in ctx.iter() {
      if let RelogTerm::Var(_) = k { continue; }
      if let RelogTerm::Var(_) = x { continue; }
      let mut ctx = ctx.clone();
      ctx.remove(k).unwrap();
      let u = relog_unify(&mut ctx, k.clone(), x.clone());
      if u != RelogTerm::Reject {
         let r = relog_apply(&mut ctx, v.clone());
         return relog_reify(&mut ctx, r);
      }
   }
   x.clone()
}

fn relog_unify(ctx: &mut HashMap<RelogTerm,RelogTerm>, l: RelogTerm, r: RelogTerm) -> RelogTerm {
   match (relog_apply(ctx, l),relog_apply(ctx, r)) {
      (l,r) if l==r => { l.clone() },
      (RelogTerm::Var(l),r) => {
         ctx.insert(RelogTerm::Var(l), r.clone());
         r.clone()
      },
      (l,RelogTerm::Var(r)) => {
         ctx.insert(RelogTerm::Var(r), l.clone());
         l.clone()
      },
      (RelogTerm::Compound(lh,lt),RelogTerm::Compound(rh,rt)) if lh==rh && lt.len()==rt.len() => {
         let mut us = Vec::new();
         for (lx,rx) in std::iter::zip(lt,rt) {
            us.push(relog_unify(ctx, lx.clone(), rx.clone()));
         }
         if us.contains(&RelogTerm::Reject) { return RelogTerm::Reject; }
         RelogTerm::Compound(lh.clone(),us)
      },
      _ => RelogTerm::Reject,
   }
}

fn relog_reify(ctx: &mut HashMap<RelogTerm,RelogTerm>, x: RelogTerm) -> RelogTerm {
   match x {
      RelogTerm::Reject => { RelogTerm::Reject },
      RelogTerm::Atomic(x) => { RelogTerm::Atomic(x.clone()) },
      RelogTerm::Var(_) => {
         let mut ctx = ctx.clone();
         if let Some(r) = ctx.remove(&x) {
            relog_reify(&mut ctx,r.clone())
         } else { x.clone() }
      },
      RelogTerm::Compound(x,xs) => {
         RelogTerm::Compound( x.clone(), xs.into_iter().map(|x| relog_reify(ctx,x)).collect::<Vec<RelogTerm>>() )
      }
   }
}

fn unpack_bindings(ctx: &mut HashMap<RelogTerm,RelogTerm>, x: RelogTerm) {
   match x {
      RelogTerm::Compound(g,gs) => {
         if g=="Bind" && gs.len()==2 {
            ctx.insert( gs[0].clone(), gs[1].clone() );
         }
         for gx in gs.iter() {
            unpack_bindings(ctx, gx.clone());
         }
      },
      _ => (),
   }
}

pub fn relog(s: &str) -> String {
   let p = parse_relog_prog(s);
   let mut ctx: HashMap<RelogTerm,RelogTerm> = p.bindings.into_iter().collect();
   for (l,r) in p.unifications {
      let x = relog_unify(&mut ctx, l.clone(), r.clone());
      if x == RelogTerm::Reject {
         return RelogTerm::Reject.to_string();
      }
      unpack_bindings(&mut ctx, x);
   }
   let r = relog_apply(&mut ctx, p.returns);   
   relog_reify(&mut ctx, r).to_string()
}

#[cfg(test)]
mod tests {
   use super::*;

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
      assert_eq!(parse_relog_prog("A<b,c>=R<d>;A<B>").to_string(), "A<b,c>=R<d>;A<B>");
      assert_eq!(parse_relog_prog("A<b,c>:=R<d>;A<B>").to_string(), "A<b,c>:=R<d>;A<B>");
      assert_eq!(parse_relog_prog("").to_string(), "!");
      assert_eq!(parse_relog_prog(";").to_string(), "!");
      assert_eq!(parse_relog_prog("x;").to_string(), "!");
      assert_eq!(parse_relog_prog("F<x>:=Bind<G<y>,y>;_=F<1>;z=G<2>;z").to_string(), "F<x>:=Bind<G<y>,y>;_=F<1>;z=G<2>;z");
   }

   #[test]
   fn substitution() {
      assert_eq!(relog("a=Int;T<a>"), "T<Int>");
   }

   #[test]
   fn unification() {
      assert_eq!(relog("A<b,C<d>>=A<Int,C<Bool>>;R<b>"), "R<Int>");
   }

   #[test]
   fn recursion() {
      assert_eq!(relog("a=B;c=C<a>;c"), "C<B>");
      assert_eq!(relog("a=B;c=C<a,a>;c"), "C<B,B>");
      relog("a=A<a,a>;a");
   }

   #[test]
   fn function() {
      assert_eq!(relog("A<b,c>:=R<c>;A<B,C>"), "R<C>");
      relog("A<b>:=A<b>;A<B>");
      relog("A<b>:=B<a>;B<a>:=A<b>;A<B>");
   }

   #[test]
   fn staged_bindings() {
      assert_eq!(relog("F<x>:=Bind<G<y>,y>;x=F<1>;x"), "Bind<G<y>,y>");
      assert_eq!(relog("F<x>:=Bind<G<y>,y>;_=F<1>;z=G<2>;z"), "2");
   }
}
