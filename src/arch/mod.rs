pub mod aarch64;
pub use aarch64::*;
mod sync;
pub use sync::*;
pub use sync::NullLock as Mutex;