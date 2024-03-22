//! Extracting data from [`Interaction`]

#[cfg(doc)]
use twilight_model::application::interaction::Interaction;

mod interaction_data;
mod option;

pub use interaction_data::ExtractInteractionData;
pub use option::{ExtractOption, ExtractOptionValue};
