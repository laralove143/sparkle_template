use twilight_model::application::interaction::modal::ModalInteractionDataActionRow;

/// Trait implemented on [`Vec<ModalInteractionDataActionRow>`] extract options
pub trait ExtractModalComponent {
    /// Extract the value of the component with the given custom ID
    ///
    /// Returns `None` if not found
    fn component(self, custom_id: &str) -> Option<String>;
}

impl ExtractModalComponent for Vec<ModalInteractionDataActionRow> {
    fn component(self, custom_id: &str) -> Option<String> {
        for action_row in self {
            if let Some(value) = action_row
                .components
                .into_iter()
                .find_map(|component| (component.custom_id == custom_id).then_some(component.value))
            {
                return value;
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use twilight_model::{
        application::interaction::modal::{
            ModalInteractionDataActionRow, ModalInteractionDataComponent,
        },
        channel::message::component::ComponentType,
    };

    use crate::extract::modal_component::ExtractModalComponent;

    #[test]
    fn test_extract_component() {
        let custom_id = "custom_id";
        let value = "value";

        assert_eq!(
            vec![ModalInteractionDataActionRow {
                components: vec![ModalInteractionDataComponent {
                    custom_id: custom_id.to_owned(),
                    kind: ComponentType::TextInput,
                    value: Some(value.to_owned()),
                }],
            }]
            .component(custom_id),
            Some(value.to_owned())
        );

        assert_eq!(
            vec![ModalInteractionDataActionRow {
                components: vec![ModalInteractionDataComponent {
                    custom_id: "a".to_owned(),
                    kind: ComponentType::TextInput,
                    value: Some(value.to_owned()),
                }],
            }]
            .component(custom_id),
            None
        );

        assert_eq!(
            vec![ModalInteractionDataActionRow {
                components: vec![ModalInteractionDataComponent {
                    custom_id: custom_id.to_owned(),
                    kind: ComponentType::TextInput,
                    value: None,
                }],
            }]
            .component(custom_id),
            None
        );
    }
}
