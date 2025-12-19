use crate::attributes::common::Common;
use crate::events::OnInput;

pub struct Input {
    pub common: Common,
    pub on_input: Option<OnInput>,
}