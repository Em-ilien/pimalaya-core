use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::{folder::sync::config::FolderSyncStrategy, serde::serde_deprecated};

serde_deprecated!(strategy, "strategy.sync", "folder.sync.filter");

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SyncConfig {
    /// Enable the synchronization of the current account with local
    /// Maildir backend features.
    pub enable: Option<bool>,

    /// Customize the root directory where the Maildir cache is
    /// saved. Defaults to `$XDG_DATA_HOME/himalaya/<account-name>`.
    pub dir: Option<PathBuf>,

    #[deprecated(since = "0.22.0", note = "use FolderConfig::sync::filter instead")]
    #[serde(default, skip_serializing, deserialize_with = "strategy_deprecated")]
    pub strategy: Option<FolderSyncStrategy>,
}
