#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AxTestResult {
    Ok,
    Failed
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AxTestExecutionMode {
    Standard = 0,
    Ignore   = 1,
    Custom   = 2,
    User     = 3,
}

// Test descriptor structure
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AxTestDescriptor {
    pub name: &'static str,
    pub module: &'static str,
    pub test_fn: fn() -> AxTestResult,
    pub should_panic: bool,
    pub execution_mode: AxTestExecutionMode,
}

impl AxTestDescriptor {
    pub const fn new(
        name: &'static str,
        module: &'static str,
        test_fn: fn() -> AxTestResult,
        should_panic: bool,
        execution_mode: AxTestExecutionMode,
    ) -> Self {
        Self {
            name,
            module,
            test_fn,
            should_panic,
            execution_mode,
        }
    }
}