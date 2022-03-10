#[cfg(target_os = "android")]
mod android;
pub mod app_memory;
mod auxv_reader;
mod dso_debug;
mod dumper_cpu_info;
pub mod errors;
pub mod maps_reader;
pub mod minidump_writer;
pub mod ptrace_dumper;
mod sections;
pub mod thread_info;

pub use maps_reader::LINUX_GATE_LIBRARY_NAME;
