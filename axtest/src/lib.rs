#![no_std]

extern crate alloc;

mod asserts;
mod executor;
mod framework;

pub use axtest_macros::{def_mod, def_test};
pub use executor::{init, AxTestExecutor, AxTestInitBuilder, InlineExecutor};
pub use framework::{AxTestDescriptor, AxTestExecutionMode, AxTestResult};
pub use framework::{TestRunResult, TestSummary};
