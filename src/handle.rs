//! Responding to interactions concisely
//!
//! [`InteractionHandle::respond`] centralizes creating the first response and
//! creating a followup by tracking whether the interaction was responded to.
//!
//! # Example
//!
//! ```rust
//! # use sparkle_interactions::{builder::InteractionResponseBuilder, InteractionHandle};
//! # use twilight_http::Client;
//! # use twilight_model::{
//! #     application::interaction::{Interaction, InteractionType},
//! #     http::interaction::InteractionResponseData,
//! #     id::Id,
//! # };
//! #
//! # async fn foo() {
//! # let interaction = Interaction {
//! #     app_permissions: None,
//! #     application_id: Id::new(1),
//! #     channel: None,
//! #     #[allow(deprecated)]
//! #     channel_id: None,
//! #     data: None,
//! #     guild_id: None,
//! #     guild_locale: None,
//! #     id: Id::new(1),
//! #     kind: InteractionType::ApplicationCommand,
//! #     locale: None,
//! #     member: None,
//! #     message: None,
//! #     token: String::new(),
//! #     user: None,
//! # };
//! #
//! # let response_data = InteractionResponseData {
//! #     allowed_mentions: None,
//! #     attachments: None,
//! #     choices: None,
//! #     components: None,
//! #     content: None,
//! #     custom_id: None,
//! #     embeds: None,
//! #     flags: None,
//! #     title: None,
//! #     tts: None,
//! # };
//! #
//! # let client = Client::new("a".to_owned());
//! let interaction_client = client.interaction(interaction.application_id);
//! let handle = InteractionHandle::new(&interaction_client, interaction.id, &interaction.token);
//!
//! handle
//!     .respond(InteractionResponseBuilder::send_message(response_data.clone()).build())
//!     .await?;
//! // interaction response is created here
//!
//! handle
//!     .respond(InteractionResponseBuilder::send_message(response_data))
//!     .await?;
//! // followup message is created here
//! # }
//! ```

use std::{
    fmt::{Display, Formatter},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use twilight_http::{client::InteractionClient, Response};
use twilight_model::{
    channel::Message,
    http::interaction::InteractionResponse,
    id::{marker::InteractionMarker, Id},
};
use twilight_validate::message::MessageValidationError;

/// Errors returned while responding to interactions
#[derive(Debug)]
pub enum Error {
    /// An error was returned by [`twilight_http`]
    Http(twilight_http::Error),
    /// A [`MessageValidationError`] was returned
    MessageValidation(MessageValidationError),
}

impl Display for Error {
    fn fmt(&self, #[allow(clippy::min_ident_chars)] f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http(err) => err.fmt(f),
            Self::MessageValidation(err) => err.fmt(f),
        }
    }
}

impl From<twilight_http::Error> for Error {
    fn from(err: twilight_http::Error) -> Self {
        Self::Http(err)
    }
}

impl From<MessageValidationError> for Error {
    fn from(err: MessageValidationError) -> Self {
        Self::MessageValidation(err)
    }
}

impl std::error::Error for Error {}

/// Struct for responding to interactions
///
/// Holds stateful data to create a valid response to an interaction.
///
/// It can be cloned safely without losing stateful data, it is also
/// thread-safe.
#[derive(Clone, Debug)]
pub struct InteractionHandle<'a> {
    client: &'a InteractionClient<'a>,
    id: Id<InteractionMarker>,
    token: &'a str,
    is_responded: Arc<AtomicBool>,
}

impl<'a> InteractionHandle<'a> {
    /// Create a new handle for an interaction
    ///
    /// # Warnings
    ///
    /// Create only one handle per interaction. Otherwise, the interaction's
    /// state will be lost
    #[must_use]
    pub fn new(
        client: &'a InteractionClient<'a>,
        interaction_id: Id<InteractionMarker>,
        token: &'a str,
    ) -> Self {
        Self {
            client,
            id: interaction_id,
            token,
            is_responded: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Respond to the interaction with the given response.
    ///
    /// If this is the first response, it creates a new response, otherwise
    /// it creates a followup response.
    ///
    /// Returns `None` if this is the first response, otherwise returns the
    /// response.
    ///
    /// There is a builder for [`InteractionResponse`] at
    /// [`InteractionResponseBuilder`]
    ///
    /// # Errors
    ///
    /// Returns [`Error::MessageValidation`] if response isn't a valid followup
    /// message.
    ///
    /// Returns [`Error::Http`] if a request failed.
    ///
    /// [`InteractionResponseBuilder`]: crate::builder::InteractionResponseBuilder
    pub async fn respond(
        &self,
        response: InteractionResponse,
    ) -> Result<Option<Response<Message>>, Error> {
        if self.is_responded() {
            self.create_followup(response).await.map(Some)
        } else {
            self.client
                .create_response(self.id, self.token, &response)
                .await?;

            self.set_is_responded(true);

            Ok(None)
        }
    }

    async fn create_followup(
        &self,
        response: InteractionResponse,
    ) -> Result<Response<Message>, Error> {
        let mut create_followup = self.client.create_followup(self.token);

        let Some(data) = response.data else {
            return Ok(create_followup.await?);
        };

        if let Some(attachments) = &data.attachments {
            create_followup = create_followup.attachments(attachments)?;
        }

        if let Some(components) = &data.components {
            create_followup = create_followup.components(components)?;
        }

        if let Some(content) = &data.content {
            create_followup = create_followup.content(content)?;
        }

        if let Some(embeds) = &data.embeds {
            create_followup = create_followup.embeds(embeds)?;
        }

        if let Some(flags) = data.flags {
            create_followup = create_followup.flags(flags);
        }

        if let Some(tts) = data.tts {
            create_followup = create_followup.tts(tts);
        }

        Ok(create_followup.await?)
    }

    fn is_responded(&self) -> bool {
        self.is_responded.load(Ordering::Acquire)
    }

    fn set_is_responded(&self, value: bool) {
        self.is_responded.store(value, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use twilight_http::Client;
    use twilight_model::id::Id;

    use crate::InteractionHandle;

    #[test]
    fn test_data_integrity() {
        let client = Client::new("a".to_owned());
        let interaction_client = client.interaction(Id::new(1));
        let handle = InteractionHandle::new(&interaction_client, Id::new(1), "a");
        let handle_clone = handle.clone();

        handle.set_is_responded(true);
        assert_eq!(handle.is_responded(), handle_clone.is_responded());
    }
}
