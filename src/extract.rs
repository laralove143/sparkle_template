//! Extracting data from [`Interaction`]

#[cfg(doc)]
use twilight_model::application::interaction::Interaction;

mod interaction_data;
mod modal_component;
mod option;

pub use interaction_data::ExtractInteractionData;
pub use modal_component::ExtractModalComponent;
pub use option::{ExtractOption, ExtractOptionValue};
