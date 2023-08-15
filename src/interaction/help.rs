use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::interaction::Handle;

#[derive(CommandModel, CreateCommand)]
#[command(name = "help", desc = "Get info about the bot")]
pub struct Command;

impl Handle<'_, Command> {
    pub async fn handle(self) -> Result<(), anyhow::Error> {
        Ok(())
    }
}
