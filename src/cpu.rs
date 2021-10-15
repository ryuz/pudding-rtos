//#![allow(unused_imports)]

#[cfg(all(not(feature = "std"), target_arch = "x86_64"))]
pub mod x86_64;
#[cfg(all(not(feature = "std"), target_arch = "x86_64"))]
pub use x86_64::*;

#[cfg(all(not(feature = "std"), target_arch = "arm"))]
pub mod arm;
#[cfg(all(not(feature = "std"), target_arch = "arm"))]
pub use arm::*;

#[cfg(any(feature = "std", not(any(target_arch = "x86_64", target_arch = "arm"))))]
pub mod dummy;
#[allow(unused_imports)]
#[cfg(any(feature = "std", not(any(target_arch = "x86_64", target_arch = "arm"))))]
pub use dummy::*;

