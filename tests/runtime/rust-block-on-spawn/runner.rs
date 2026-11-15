//@ wasmtime-flags = '-Wcomponent-model-async'

include!(env!("BINDINGS"));

use crate::test::rust_block_on_spawn::i::ping;
use futures::channel::oneshot;
use std::cell::Cell;
use std::rc::Rc;
use wit_bindgen::{block_on, spawn_local};

struct Component;

export!(Component);

impl Guest for Component {
    async fn run() {
        task_spawned_before_block_on_stays_with_its_spawner().await;
        task_spawned_within_block_on_is_waited_for();
    }
}

/// A task spawned just before a `block_on`, in the same poll, belongs to the
/// computation that spawned it: `block_on` returns without waiting for it, and
/// it keeps running afterwards.
async fn task_spawned_before_block_on_stays_with_its_spawner() {
    let (release, released) = oneshot::channel::<u32>();
    let waiting = spawn_local(async move { released.await.unwrap() });

    assert_eq!(block_on(async { 42 }), 42);

    release.send(7).unwrap();
    assert_eq!(waiting.await, Some(7));
}

/// A task spawned within a `block_on` keeps it from returning until the task
/// finishes, even if its `JoinHandle` is dropped.
fn task_spawned_within_block_on_is_waited_for() {
    let finished = Rc::new(Cell::new(false));
    block_on({
        let finished = finished.clone();
        async move {
            drop(spawn_local(async move {
                ping().await;
                finished.set(true);
            }));
        }
    });
    assert!(finished.get());
}
