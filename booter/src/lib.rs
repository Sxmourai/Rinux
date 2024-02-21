pub mod command;
pub use command::*;
pub mod args;
#[cfg(not(debug_assertions))]
pub const PROFILE: Profile = Profile::Release;
#[cfg(debug_assertions)]
#[cfg(debug_assertions)]
pub const PROFILE: Profile = Profile::Dev;

pub enum Profile {
    Release,
    Dev,
}
impl Profile {
    pub fn as_profile(self) -> &'static str {
        match self {
            Profile::Release => "release",
            Profile::Dev => "dev",
        }
    }
    pub fn as_path(self) -> &'static str {
        match self {
            Profile::Release => "release",
            Profile::Dev => "debug",
        }
    }
}

/// https://users.rust-lang.org/t/concatenate-const-strings/51712/4
#[macro_export]
macro_rules! combine {
    ($A:expr, $B:expr) => {{
        const LEN: usize = $A.len() + $B.len();
        const fn combine(a: &'static str, b: &'static str) -> [u8; LEN] {
            let mut out = [0u8; LEN];
            out = copy_slice(a.as_bytes(), out, 0);
            out = copy_slice(b.as_bytes(), out, a.len());
            out
        }
        const fn copy_slice(input: &[u8], mut output: [u8; LEN], offset: usize) -> [u8; LEN] {
            let mut index = 0;
            loop {
                output[offset + index] = input[index];
                index += 1;
                if index == input.len() {
                    break;
                }
            }
            output
        }
        combine($A, $B)
    }};
}
