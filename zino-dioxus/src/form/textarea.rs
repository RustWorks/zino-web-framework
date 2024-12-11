use crate::class::Class;
use dioxus::prelude::*;
use zino_core::SharedString;

/// The multiline textarea and its variations.
pub fn Textarea(props: TextareaProps) -> Element {
    rsx! {
        textarea {
            class: "{props.class}",
            class: if !props.color.is_empty() { "is-{props.color}" },
            class: if !props.size.is_empty() { "is-{props.size}" },
            class: if !props.state.is_empty() { "is-{props.state}" },
            value: props.initial_value,
            onchange: move |event| {
                if let Some(handler) = props.on_change.as_ref() {
                    handler.call(event.value());
                }
            },
            oninput: move |event| {
                if let Some(handler) = props.on_input.as_ref() {
                    handler.call(event.value());
                }
            },
            ..props.attributes,
        }
    }
}

/// The [`Textarea`] properties struct for the configuration of the component.
#[derive(Clone, PartialEq, Props)]
pub struct TextareaProps {
    /// The class attribute for the component.
    #[props(into, default = "textarea")]
    pub class: Class,
    /// The color of the input: `primary` | `link` | `info` | `success` | `warning` | `danger`.
    #[props(into, default)]
    pub color: SharedString,
    /// The size of the input: `small` | `normal` | `medium` | `large`.
    #[props(into, default)]
    pub size: SharedString,
    /// The state of the input: `hovered` | `focused` | `loading`.
    #[props(into, default)]
    pub state: SharedString,
    /// The initial value of the textarea.
    #[props(into, default)]
    pub initial_value: String,
    /// An event handler to be called when the textarea state is changed.
    pub on_change: Option<EventHandler<String>>,
    /// An event handler to be called when inputing.
    pub on_input: Option<EventHandler<String>>,
    /// Spreading the props of the `textarea` element.
    #[props(extends = textarea)]
    attributes: Vec<Attribute>,
}
