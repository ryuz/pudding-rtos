#[cfg(feature = "pl390")]
pub mod pl390;
#[cfg(feature = "pl390")]
pub use pl390::*;

#[cfg(not(feature = "pl390"))]
pub mod dummy;

#[cfg(not(feature = "pl390"))]
pub use dummy::*;
