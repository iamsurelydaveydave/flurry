pub mod column;
pub mod row;
pub mod text;
pub mod button;
pub mod input;

pub enum Element {
    Column(column::Column),
    Row(row::Row),
    Text(text::Text),
    Button(button::Button),
    Input(input::Input),
}