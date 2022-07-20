use std::fs;

use deno_core::op;
use deno_core::Extension;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;

// This is a hack to make the `#[op]` macro work with
// deno_core examples.
// You can remove this:
// use deno_core::*;

gflags::define! {
  -f, --file = false
}
gflags::define! {
  -i, --input = false
}

#[op]
fn op_sum(nums: Vec<f64>) -> Result<f64, deno_core::error::AnyError> {
  // Sum inputs
  let sum = nums.iter().fold(0.0, |a, v| a + v);
  // return as a Result<f64, AnyError>
  Ok(sum)
}

fn main() {
  let args = gflags::parse();
  if args.len() >= 1 {
    let code = fs::read_to_string(args[0])
    .expect("Something went wrong reading the file");
    runtime(code, args[0]);
  } else {
    //help
  }
  
}
fn runtime(code: String, name: &str) {
  // Build a deno_core::Extension providing custom ops
  let ext = Extension::builder()
    .ops(vec![
      // An op for summing an array of numbers
      // The op-layer automatically deserializes inputs
      // and serializes the returned Result & value
      op_sum::decl(),
    ])
    .build();

  // Initialize a runtime instance
  let mut runtime = JsRuntime::new(RuntimeOptions {
    extensions: vec![ext],
    ..Default::default()
  });

  // Now we see how to invoke the op we just defined. The runtime automatically
  // contains a Deno.core object with several functions for interacting with it.
  // You can find its definition in core.js.
  runtime
    .execute_script(
      name,
      &code,
    )
    .unwrap();
}