use latexml_runner::Harness;
use rand::prelude::*;
use std::time::Instant;

fn runner_helper(input_file: &str, output_file: &str, log_file: &str) {
  let start_test = Instant::now();
  let from_port: u16 = thread_rng().gen_range(11000, 20000);
  let harness_result = Harness::new(
    from_port,
    0,
    [
      ("whatsin", "math"),
      ("whatsout", "math"),
      ("preload", "article.cls"),
      ("preload", "amsmath.sty"),
    ]
    .iter()
    .map(|(x, y)| (x.to_string(), y.to_string()))
    .collect(),
  );
  assert!(harness_result.is_ok(), "{:?}", harness_result);
  let mut harness = harness_result.unwrap();

  let rel_input_file = format!("tests/data/{}", input_file);
  let rel_output_file = format!("tests/scratch/{}", output_file);
  let rel_log_file = format!("tests/scratch/{}", log_file);
  let result = harness.convert_file(&rel_input_file, &rel_output_file, &rel_log_file);
  assert!(result.is_ok(), "{:?}", result);
  eprintln!(
    "-- {} test took {:?}ms",
    input_file,
    start_test.elapsed().as_millis()
  );
}

#[test]
#[ignore]
fn convert_mixed_severity() {
  runner_helper("mixed_1m.csv", "mixed_1m_result.csv", "mixed_1m.log");
}
