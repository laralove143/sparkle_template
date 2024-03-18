//! Extracting data from [`Interaction`]

use twilight_model::application::interaction::{
    application_command::CommandData, message_component::MessageComponentInteractionData,
    modal::ModalInteractionData, Interaction, InteractionData,
};

use crate::Error;

/// Trait implemented on [`Interaction`] to extract [`InteractionData`] variants
/// without using pattern matching
pub trait ExtractInteractionData {
    /// Extract [`InteractionData`] from an interaction
    ///
    /// # Errors
    ///
    /// Returns [`Error::MissingInteractionData`] if the interaction is a ping
    /// interaction
    fn data(self) -> Result<InteractionData, Error>;

    /// Extract [`CommandData`] from an interaction
    ///
    /// # Errors
    ///
    /// Returns [`Error::MissingCommandData`] if the interaction
    /// is not an application command
    fn command_data(self) -> Result<CommandData, Error>;

    /// Extract [`MessageComponentInteractionData`] from an interaction
    ///
    /// # Errors
    ///
    /// Returns [`Error::MissingComponentData`] if the interaction is not a
    /// message component
    fn component_data(self) -> Result<MessageComponentInteractionData, Error>;

    /// Extract [`ModalInteractionData`] from an interaction
    ///
    /// # Errors
    ///
    /// Returns [`Error::MissingModalData`] if the interaction is not a modal
    /// submit
    fn modal_data(self) -> Result<ModalInteractionData, Error>;
}

impl ExtractInteractionData for Interaction {
    fn data(self) -> Result<InteractionData, Error> {
        self.data.ok_or(Error::MissingInteractionData)
    }

    fn command_data(self) -> Result<CommandData, Error> {
        if let InteractionData::ApplicationCommand(data) = self.data()? {
            Ok(*data)
        } else {
            Err(Error::MissingCommandData)
        }
    }

    fn component_data(self) -> Result<MessageComponentInteractionData, Error> {
        if let InteractionData::MessageComponent(data) = self.data()? {
            Ok(data)
        } else {
            Err(Error::MissingComponentData)
        }
    }

    fn modal_data(self) -> Result<ModalInteractionData, Error> {
        if let InteractionData::ModalSubmit(data) = self.data()? {
            Ok(data)
        } else {
            Err(Error::MissingModalData)
        }
    }
}
