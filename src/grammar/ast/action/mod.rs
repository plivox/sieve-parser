pub mod discard;
pub mod fileinto;
pub mod keep;
pub mod redirect;
pub mod vacation;

pub use discard::ActionDiscard;
pub use fileinto::ActionFileinto;
pub use keep::ActionKeep;
pub use redirect::ActionRedirect;
pub use vacation::ActionVacation;
