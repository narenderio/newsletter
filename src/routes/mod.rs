//! src/routes/mod.rs
mod health_check;
mod subscriptions;
// New module!
mod newsletters;

// New module!
mod subscriptions_confirm;
pub use health_check::*;
pub use newsletters::*;
pub use subscriptions::*;
pub use subscriptions_confirm::*;
// New module!
mod home;
pub use home::*;
mod login;
pub use login::*;
