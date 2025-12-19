use crate::elements::Element;

pub struct UiNode {
    pub element: Element,
    pub children: Vec<UiNode>,
}