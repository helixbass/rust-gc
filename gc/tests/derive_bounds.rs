#![cfg_attr(feature = "nightly", feature(specialization))]

use gc_derive::{Finalize, Trace};
use gc::Gc;

// This impl should *not* require T: Trace.
#[derive(Finalize, Trace)]
struct Thunk<T>(fn() -> T);

struct NotTrace;

#[test]
fn test_derive_bounds() {
    let _: Gc<Thunk<NotTrace>> = Gc::new(Thunk(|| NotTrace));
}
