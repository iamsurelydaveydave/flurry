use crate::attributes::common::Common;
use crate::events::OnClick;

pub struct Button {
    pub common: Common,
    pub on_click: Option<OnClick>,
}