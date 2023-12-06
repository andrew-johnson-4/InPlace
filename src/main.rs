/*

Copyright 2023 - Andrew Johnson

This code and all related intellectual property is available under the terms of
the attached permissive MIT license. This license is intended only to protect
the future development of the project while otherwise allowing people to use
the code and IP as they would like. Please, just be nice.

*/

use relog::relog;

pub fn main() {
   for a in std::env::args().skip(1) {
      println!("{}", relog(&a));
   }
}
