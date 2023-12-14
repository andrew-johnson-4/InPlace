
use std::process::Command;

#[test]
fn _123() {
   let mut run_123 = Command::new("relog");
   run_123.arg("tests/123.rl");
   assert_eq!(
      String::from_utf8_lossy(&run_123.output().expect("failed to execute process").stdout),
      "123\n"
   );
}
