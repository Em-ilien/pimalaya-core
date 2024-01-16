//! Module dedicated to account management.
//!
//! This module contains everything related to account configuration,
//! plus everything you need to synchronize a remote account using a
//! local Maildir backend. It also contains common code related to
//! PGP.

pub mod config;
#[cfg(feature = "account-discovery")]
pub mod discover;
#[cfg(feature = "sync")]
pub mod sync;
