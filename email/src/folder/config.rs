use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::list::config::FolderListConfig;
#[cfg(feature = "account-sync")]
use super::sync::config::FolderSyncConfig;

/// The folder configuration.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FolderConfig {
    /// Define custom folder aliases.
    ///
    /// Aliases are resolved when calling backend features. There are
    /// 4 special aliases that map to [`super::FolderKind`]: inbox,
    /// draft(s), sent and trash. Other aliases map to folder names.
    ///
    /// Note: folder aliases are case-insensitive.
    pub aliases: Option<HashMap<String, String>>,

    /// The configuration dedicated to folder listing.
    pub list: Option<FolderListConfig>,

    #[cfg(feature = "account-sync")]
    /// The configuration dedicated to folder synchronization.
    pub sync: Option<FolderSyncConfig>,
}
