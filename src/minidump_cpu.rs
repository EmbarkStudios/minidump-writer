cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        pub mod amd64;
        pub use amd64 as imp;
        pub type RawContextCPU = amd64::MDRawContextAMD64;
    } else if #[cfg(target_arch = "x86")] {
        pub mod x86;
        pub use x86 as imp;
        pub type RawContextCPU = amd64::MDRawContextX86;
    } else if #[cfg(target_arch = "arm")] {
        pub mod arm;
        pub use arm as imp;
        pub type RawContextCPU = arm::MDRawContextARM;
    } else if #[cfg(target_arch = "aarch64")] {
        pub mod aarch64;
        pub use aarch64 as imp;

        compile_error!("flesh me out");
        //pub type RawContextCPU = aarch64::MDRawContextX86;
    } else if #[cfg(target_arch = "mips")] {
        compile_error!("flesh me out");
    } else {
        compile_error!("unsupported target architecture");
    }
}
