//! # Account synchronization
//!
//! Module dedicated to synchronization of folders and emails
//! belonging to an account. The main structure of this module is
//! [`AccountSyncBuilder`], which allows you to synchronize a given
//! backend with a local Maildir one, and therefore enables offline
//! support for this backend.

pub mod config;

use std::sync::Arc;

use crate::{
    backend::{BackendBuilder, BackendContextBuilder},
    maildir::{config::MaildirConfig, MaildirContextBuilder},
    sync::SyncBuilder,
    Result,
};

use super::config::AccountConfig;

/// The account synchronization builder.
///
/// This builder is just a wrapper around [`SyncBuilder`], where the
/// left backend builder is a pre-defined Maildir one. The aim of this
/// builder is to provide offline support for any given backend.
pub struct AccountSyncBuilder;

impl AccountSyncBuilder {
    /// Create a new account synchronization builder.
    pub fn new<R: BackendContextBuilder + 'static>(
        right_builder: BackendBuilder<R>,
    ) -> Result<SyncBuilder<MaildirContextBuilder, R>> {
        let account_config = Arc::new(AccountConfig {
            name: right_builder.account_config.name.clone() + "-cache",
            ..(*right_builder.account_config).clone()
        });

        let sync_dir = account_config.get_sync_dir()?;
        let mdir_config = Arc::new(MaildirConfig { root_dir: sync_dir });
        let ctx_builder = MaildirContextBuilder::new(mdir_config);
        let left_builder = BackendBuilder::new(account_config, ctx_builder);

        Ok(SyncBuilder::new(left_builder, right_builder))
    }
}
