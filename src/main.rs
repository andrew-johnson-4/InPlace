
use relog::relog;

pub fn main() {
   for a in std::env::args().skip(1) {
      println!("{}", relog(&a));
   }
}
