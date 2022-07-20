// Copyright 2018-2022 the Deno authors. All rights reserved. MIT license.

use deno_core::error::AnyError;
use deno_core::FsModuleLoader;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::Permissions;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::BootstrapOptions;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

gflags::define! {
  -f, --file = false
}
gflags::define! {
  -i, --input = false
}

fn get_error_class_name(e: &AnyError) -> &'static str {
  deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

#[tokio::main]
async fn main() -> Result<(), AnyError> {
  let args = gflags::parse();
  if args.len() >= 1 {
    let module_loader = Rc::new(FsModuleLoader);
    let create_web_worker_cb = Arc::new(|_| {
      todo!("Web workers are not supported in the example");
    });
    let web_worker_preload_module_cb = Arc::new(|_| {
      todo!("Web workers are not supported in the example");
    });

    let options = WorkerOptions {
      bootstrap: BootstrapOptions {
        args: vec![],
        cpu_count: 1,
        debug_flag: false,
        enable_testing_features: false,
        location: None,
        no_color: false,
        is_tty: false,
        runtime_version: "x".to_string(),
        ts_version: "x".to_string(),
        unstable: false,
        user_agent: "hello_runtime".to_string(),
      },
      extensions: vec![],
      unsafely_ignore_certificate_errors: None,
      root_cert_store: None,
      seed: None,
      source_map_getter: None,
      format_js_error_fn: None,
      web_worker_preload_module_cb,
      create_web_worker_cb,
      maybe_inspector_server: None,
      should_break_on_first_statement: false,
      module_loader,
      get_error_class_fn: Some(&get_error_class_name),
      origin_storage_dir: None,
      blob_store: BlobStore::default(),
      broadcast_channel: InMemoryBroadcastChannel::default(),
      shared_array_buffer_store: None,
      compiled_wasm_module_store: None,
      stdio: Default::default(),
    };

    let js_path =
      Path::new(args[0]);
      //Path::new(env!("CARGO_MANIFEST_DIR")).join(args[0]);
    let main_module = deno_core::resolve_path(&js_path.to_string_lossy())?;
    let permissions = Permissions::allow_all();

    let mut worker = MainWorker::bootstrap_from_options(
      main_module.clone(),
      permissions,
      options,
    );
    worker.execute_main_module(&main_module).await?;
    worker.run_event_loop(false).await?;
  } else {
    //help
  }
  Ok(())
}


// use std::fs;

// use deno_core::op;
// use deno_core::Extension;
// use deno_core::JsRuntime;
// use deno_core::RuntimeOptions;

// // This is a hack to make the `#[op]` macro work with
// // deno_core examples.
// // You can remove this:
// // use deno_core::*;

// 

// #[op]
// fn op_sum(nums: Vec<f64>) -> Result<f64, deno_core::error::AnyError> {
//   // Sum inputs
//   let sum = nums.iter().fold(0.0, |a, v| a + v);
//   // return as a Result<f64, AnyError>
//   Ok(sum)
// }

// fn main() {
  
// }
// fn runtime(code: String, name: &str) {
//   // Build a deno_core::Extension providing custom ops
//   let ext = Extension::builder()
//     .ops(vec![
//       // An op for summing an array of numbers
//       // The op-layer automatically deserializes inputs
//       // and serializes the returned Result & value
//       op_sum::decl(),
//     ])
//     .build();

//   // Initialize a runtime instance
//   let mut runtime = JsRuntime::new(RuntimeOptions {
//     extensions: vec![ext],
//     ..Default::default()
//   });

//   // Now we see how to invoke the op we just defined. The runtime automatically
//   // contains a Deno.core object with several functions for interacting with it.
//   // You can find its definition in core.js.
//   runtime
//     .execute_script(
//       name,
//       &code,
//     )
//     .unwrap();
// }