
use relog::relog;

#[test]
fn perf() {
   let contents = std::fs::read_to_string("tests/perf.rl")
         .expect("Could not read file: tests/perf.rl");
   assert_eq!(
      relog(false, &contents),
      "123xyz"
   );
}
