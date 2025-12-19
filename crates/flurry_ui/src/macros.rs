use crate::node::UiNode;
use crate::elements::Element;
use crate::attributes::{layout::Layout, common::Common};
use crate::events::{OnClick, OnInput};

/// Main UI macro - converts declarative syntax into UiNode trees
pub macro ui($($tokens:tt)*) {
    parse_ui_element!($($tokens)*)
}

/// Internal macro for parsing individual UI elements
macro_rules! parse_ui_element {
    // Column with attributes and children
    (column($($attr:ident = $value:expr),*) { $($children:tt)* }) => {
        UiNode {
            element: Element::Column($crate::elements::column::Column {
                layout: Layout {
                    padding: parse_attribute!(padding, $($attr = $value),*),
                    gap: parse_attribute!(gap, $($attr = $value),*),
                },
            }),
            children: vec![$(parse_ui_element!($children))*],
        }
    };
    
    // Row with attributes and children
    (row($($attr:ident = $value:expr),*) { $($children:tt)* }) => {
        UiNode {
            element: Element::Row($crate::elements::row::Row {
                layout: Layout {
                    padding: parse_attribute!(padding, $($attr = $value),*),
                    gap: parse_attribute!(gap, $($attr = $value),*),
                },
            }),
            children: vec![$(parse_ui_element!($children))*],
        }
    };
    
    // Text element
    (text($content:literal)) => {
        UiNode {
            element: Element::Text($crate::elements::text::Text {
                content: $content,
                common: Common {
                    id: None,
                    disabled: false,
                    hidden: false,
                },
            }),
            children: vec![],
        }
    };
    
    // Button with children
    (button($($attr:ident = $value:expr),*) { $($children:tt)* }) => {
        UiNode {
            element: Element::Button($crate::elements::button::Button {
                common: Common {
                    id: None,
                    disabled: false,
                    hidden: false,
                },
                on_click: parse_event!(on_click, $($attr = $value),*),
            }),
            children: vec![$(parse_ui_element!($children))*],
        }
    };
    
    // Input element
    (input($($attr:ident = $value:expr),*)) => {
        UiNode {
            element: Element::Input($crate::elements::input::Input {
                common: Common {
                    id: None,
                    disabled: false,
                    hidden: false,
                },
                on_input: parse_event!(on_input, $($attr = $value),*),
            }),
            children: vec![],
        }
    };
}

/// Helper macro for parsing attributes
macro_rules! parse_attribute {
    ($target:ident, $target:ident = $value:expr, $($rest:tt)*) => {
        $value
    };
    ($target:ident, $other:ident = $value:expr, $($rest:tt)*) => {
        parse_attribute!($target, $($rest)*)
    };
    ($target:ident,) => {
        0  // Default value
    };
}

/// Helper macro for parsing events
macro_rules! parse_event {
    (on_click, on_click = $value:expr, $($rest:tt)*) => {
        Some(OnClick($value))
    };
    (on_input, on_input = $value:expr, $($rest:tt)*) => {
        Some(OnInput($value))
    };
    ($target:ident, $other:ident = $value:expr, $($rest:tt)*) => {
        parse_event!($target, $($rest)*)
    };
    ($target:ident,) => {
        None
    };
}

// Re-export the internal macros for use by the public ui! macro
pub use parse_ui_element;
pub use parse_attribute;
pub use parse_event;