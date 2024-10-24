use async_trait::async_trait;
use imap_next::imap_types::sequence::{Sequence, SequenceSet};
use tracing::{debug, info};
use utf7_imap::encode_utf7_imap as encode_utf7;

use super::{Flags, SetFlags};
use crate::{envelope::Id, imap::ImapContext, AnyResult, Error};

#[derive(Clone, Debug)]
pub struct SetImapFlags {
    ctx: ImapContext,
}

impl SetImapFlags {
    pub fn new(ctx: &ImapContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    pub fn new_boxed(ctx: &ImapContext) -> Box<dyn SetFlags> {
        Box::new(Self::new(ctx))
    }

    pub fn some_new_boxed(ctx: &ImapContext) -> Option<Box<dyn SetFlags>> {
        Some(Self::new_boxed(ctx))
    }
}

#[async_trait]
impl SetFlags for SetImapFlags {
    async fn set_flags(&self, folder: &str, id: &Id, flags: &Flags) -> AnyResult<()> {
        info!("setting imap flag(s) {flags} to envelope {id} from folder {folder}");

        let mut client = self.ctx.client().await;
        let config = &client.account_config;

        let folder = config.get_folder_alias(folder);
        let folder_encoded = encode_utf7(folder.clone());
        debug!("utf7 encoded folder: {folder_encoded}");

        let uids: SequenceSet = match id {
            Id::Single(id) => Sequence::try_from(id.as_str())
                .map_err(Error::ParseSequenceError)?
                .into(),
            Id::Multiple(ids) => ids
                .iter()
                .filter_map(|id| {
                    let seq = Sequence::try_from(id.as_str());

                    if let Err(err) = &seq {
                        debug!(?id, ?err, "skipping invalid sequence");
                    }

                    seq.ok()
                })
                .collect::<Vec<_>>()
                .try_into()
                .map_err(Error::ParseSequenceError)?,
        };

        client.select_mailbox(&folder_encoded).await?;
        client.set_flags(uids, flags.to_imap_flags_iter()).await?;

        Ok(())
    }
}
