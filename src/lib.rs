// Library facade so integration tests under tests/ can import internals.
// The binary entry point lives in src/main.rs and uses these modules via the
// implicit binary-side `crate::` prefix instead of the lib name.

pub mod cli;
pub mod db;
pub mod profile;
pub mod render;
pub mod reset;
pub mod sched;
pub mod ui;
pub mod util;
pub mod vault;
