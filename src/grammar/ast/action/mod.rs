pub mod discard;
pub mod fileinto;
pub mod keep;
pub mod notify;
pub mod redirect;
pub mod reject;
pub mod vacation;

pub use discard::ActionDiscard;
pub use fileinto::ActionFileinto;
pub use keep::ActionKeep;
pub use notify::ActionNotify;
pub use redirect::ActionRedirect;
pub use reject::ActionReject;
pub use vacation::ActionVacation;
