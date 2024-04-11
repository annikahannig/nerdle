
use std::future::Future;

use anyhow::Result;
use wasm_bindgen_futures::spawn_local;

use crate::debug::log;


// Spawn an async task on the current thread.
// The task must return an anyhow Result.
pub fn spawn<F>(future: F)
where
    F: Future<Output = Result<()>> + 'static,
{
    spawn_local(async move {
        match future.await {
            Ok(()) => {}
            Err(err) => log!("Failed: {:?}", err),
        }
    });
}


