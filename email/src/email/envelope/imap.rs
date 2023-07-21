//! Module dedicated to IMAP email envelopes.
//!
//! This module contains envelope-related mapping functions from the
//! [imap] crate types.

use imap::{
    extensions::sort::SortCriterion,
    types::{Fetch, Fetches},
};
use log::{debug, warn};
use std::{ops::Deref, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("cannot get uid of imap envelope {0}: uid is missing")]
    GetUidMissingError(u32),
}

use crate::{
    backend,
    email::{Envelope, Envelopes, Flags, Message},
    Result,
};

impl Envelopes {
    pub fn from_imap_fetches(fetches: Fetches) -> Self {
        fetches
            .iter()
            .rev()
            .filter_map(|envelope| match Envelope::from_imap_fetch(envelope) {
                Ok(envelope) => Some(envelope),
                Err(err) => {
                    warn!("cannot build imap envelope: {err}");
                    debug!("cannot build imap envelope: {err:?}");
                    None
                }
            })
            .collect()
    }
}

impl Envelope {
    pub fn from_imap_fetch(fetch: &Fetch) -> Result<Self> {
        let id = fetch
            .uid
            .ok_or(Error::GetUidMissingError(fetch.message))?
            .to_string();

        let flags = Flags::from_imap_fetch(fetch);

        // parse a fake message from the fetch header in order to
        // extract the envelope
        let msg: Message = fetch
            .header()
            .expect("Header should be included in the IMAP fetch")
            .into();

        Ok(Envelope::from_msg(id, flags, msg))
    }
}

/// The IMAP envelope sort criteria. It is just a wrapper around
/// [`imap::extensions::sort::SortCriterion`].
pub struct SortCriteria<'a>(Vec<SortCriterion<'a>>);

impl<'a> Deref for SortCriteria<'a> {
    type Target = Vec<SortCriterion<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> FromIterator<SortCriterion<'a>> for SortCriteria<'a> {
    fn from_iter<T: IntoIterator<Item = SortCriterion<'a>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl FromStr for SortCriteria<'_> {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self> {
        s.split_whitespace()
            .map(|s| match s.trim() {
                "arrival:asc" | "arrival" => Ok(SortCriterion::Arrival),
                "arrival:desc" => Ok(SortCriterion::Reverse(&SortCriterion::Arrival)),
                "cc:asc" | "cc" => Ok(SortCriterion::Cc),
                "cc:desc" => Ok(SortCriterion::Reverse(&SortCriterion::Cc)),
                "date:asc" | "date" => Ok(SortCriterion::Date),
                "date:desc" => Ok(SortCriterion::Reverse(&SortCriterion::Date)),
                "from:asc" | "from" => Ok(SortCriterion::From),
                "from:desc" => Ok(SortCriterion::Reverse(&SortCriterion::From)),
                "size:asc" | "size" => Ok(SortCriterion::Size),
                "size:desc" => Ok(SortCriterion::Reverse(&SortCriterion::Size)),
                "subject:asc" | "subject" => Ok(SortCriterion::Subject),
                "subject:desc" => Ok(SortCriterion::Reverse(&SortCriterion::Subject)),
                "to:asc" | "to" => Ok(SortCriterion::To),
                "to:desc" => Ok(SortCriterion::Reverse(&SortCriterion::To)),
                _ => Ok(Err(backend::imap::Error::ParseSortCriterionError(
                    s.to_owned(),
                ))?),
            })
            .collect::<Result<_>>()
    }
}
