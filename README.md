wonderful_edge_cases
====================

I am in love with these.. rabbitholes

## state_storage_minimal.rs

This needs sequential executor and poof it's not unit anymore as it's got state! :((

Unit tests on mutable RwLock'd static globals just does not evaluate!

cargo test

```
running 4 tests
test state_storage_minimal::tests::handle_3 ... FAILED
test state_storage_minimal::tests::handle_456 ... FAILED
test state_storage_minimal::tests::handle_789 ... FAILED
test state_storage_minimal::tests::upsert_2 ... ok

failures:

---- state_storage_minimal::tests::handle_3 stdout ----
thread 'state_storage_minimal::tests::handle_3' panicked at 'assertion failed: `(left == right)`
  left: `[0, 1, 3]`,
 right: `[0, 1, 2, 3]`', src/state_storage_minimal.rs:42:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- state_storage_minimal::tests::handle_456 stdout ----
thread 'state_storage_minimal::tests::handle_456' panicked at 'assertion failed: `(left == right)`
  left: `[0, 1, 3, 4, 5, 6]`,
 right: `[0, 1, 4, 5, 6]`', src/state_storage_minimal.rs:50:5

---- state_storage_minimal::tests::handle_789 stdout ----
thread 'state_storage_minimal::tests::handle_789' panicked at 'assertion failed: `(left == right)`
  left: `[0, 1, 3, 4, 5, 6, 7, 8, 9]`,
 right: `[0, 1, 7, 8, 9]`', src/state_storage_minimal.rs:58:5


failures:
    state_storage_minimal::tests::handle_3
    state_storage_minimal::tests::handle_456
    state_storage_minimal::tests::handle_789

test result: FAILED. 1 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```