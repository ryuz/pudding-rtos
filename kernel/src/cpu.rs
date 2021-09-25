

#[cfg(target_arch = "x86_64")]
#[macro_use]
pub mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

#[cfg(target_arch = "arm")]
#[macro_use]
pub mod arm;
#[cfg(target_arch = "arm")]
pub use arm::*;


#[cfg(not(any(target_arch = "x86_64", target_arch = "arm")))]
#[macro_use]
pub mod dummy;
#[cfg(not(any(target_arch = "x86_64", target_arch = "arm")))]
pub use dummy::*;

