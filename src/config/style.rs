use ratatui::style::{Color, Style};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct FetchingStyle {
    primary: Color,
    secondary: Color,
    field_name: Style,
    field_value: Style,
    name: Style,
}

#[derive(Deserialize, Serialize)]
pub struct ColorPalette {}
