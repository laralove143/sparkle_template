use sparkle_convenience::{
    error::{IntoError, NoCustomError, UserError},
    interaction::{
        extract::{InteractionDataExt, InteractionExt},
        InteractionHandle,
    },
};
use twilight_interactions::command::{CommandInputData, CommandModel, CreateCommand};
use twilight_model::application::interaction::Interaction;

use crate::{err_reply, Context, Error};

mod help;

struct Handle<'ctx, T> {
    ctx: &'ctx Context,
    handle: InteractionHandle<'ctx>,
    interaction: Interaction,
    data: T,
}

impl Context {
    pub async fn create_commands(&self) -> Result<(), anyhow::Error> {
        let commands = [help::Command::create_command().into()];

        self.bot
            .interaction_client()
            .set_global_commands(&commands)
            .await?;

        Ok(())
    }

    pub async fn handle_interaction(&self, interaction: Interaction) -> Result<(), anyhow::Error> {
        let handle = self.bot.interaction_handle(&interaction);
        let err_handle = handle.clone();

        let handle_interaction_res = match interaction.name().ok()? {
            help::Command::NAME => {
                Handle {
                    ctx: self,
                    handle,
                    data: help::Command::from_interaction(CommandInputData::from(
                        interaction.data.clone().ok()?.command().ok()?,
                    ))?,
                    interaction,
                }
                .handle()
                .await
            }
            _ => Err(Error::UnknownInteraction(interaction).into()),
        };

        if let Err(err) = handle_interaction_res {
            err_handle
                .report_error(
                    err_reply(),
                    UserError::<NoCustomError>::from_anyhow_err(&err),
                )
                .await?;
        }

        Ok(())
    }
}
