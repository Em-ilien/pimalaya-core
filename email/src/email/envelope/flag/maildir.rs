//! Module dedicated to Maildir email envelope flags.
//!
//! This module contains flag-related mapping functions from the
//! [maildirpp] crate types.

use maildirs::MaildirEntry;

use super::{Flag, Flags};
use crate::{
    debug,
    email::error::{Error, Result},
};

impl TryFrom<MaildirEntry> for Flags {
    type Error = Error;

    fn try_from(entry: MaildirEntry) -> Result<Self> {
        let flags = entry
            .flags()
            .map_err(|err| Error::GetMaildirFlagsError(err, entry.path().to_owned()))?;

        let flags = flags
            .iter()
            .filter_map(|flag| match Flag::try_from(*flag) {
                Ok(flag) => Some(flag),
                Err(_err) => {
                    debug!("cannot parse maildir flag char {c}, skipping it: {_err}");
                    debug!("{_err:?}");
                    None
                }
            })
            .collect();

        Ok(flags)
    }
}

impl Flags {
    pub fn to_mdir_string(&self) -> String {
        String::from_iter(self.iter().filter_map(|flag| flag.to_opt_mdir_char()))
    }
}

impl TryFrom<maildirs::Flag> for Flag {
    type Error = Error;

    fn try_from(flag: maildirs::Flag) -> Result<Self> {
        match flag {
            maildirs::Flag::Passed => Err(Error::ParseFlagError(format!("{flag:?}"))),
            maildirs::Flag::Replied => Ok(Flag::Answered),
            maildirs::Flag::Seen => Ok(Flag::Seen),
            maildirs::Flag::Trashed => Ok(Flag::Deleted),
            maildirs::Flag::Draft => Ok(Flag::Draft),
            maildirs::Flag::Flagged => Ok(Flag::Flagged),
        }
    }
}

impl TryFrom<Flag> for maildirs::Flag {
    type Error = Error;

    fn try_from(flag: Flag) -> Result<Self> {
        match flag {
            Flag::Answered => Ok(maildirs::Flag::Replied),
            Flag::Seen => Ok(maildirs::Flag::Seen),
            Flag::Deleted => Ok(maildirs::Flag::Trashed),
            Flag::Draft => Ok(maildirs::Flag::Draft),
            Flag::Flagged => Ok(maildirs::Flag::Flagged),
            Flag::Custom(flag) => Err(Error::ParseFlagError(flag)),
        }
    }
}
