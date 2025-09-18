mod style;

use ratatui::{style::Style, widgets::BorderType as RatatuiBorderType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::config::style::FetchingStyle;

#[derive(Serialize, Deserialize, Default)]
pub enum BorderType {
    Plain,
    #[default]
    Rounded,
    Double,
    Thick,
    QuadrantInside,
    QuadrantOutside,
}

impl From<RatatuiBorderType> for BorderType {
    fn from(value: RatatuiBorderType) -> Self {
        match value {
            RatatuiBorderType::Plain => Self::Plain,
            RatatuiBorderType::Rounded => Self::Rounded,
            RatatuiBorderType::Double => Self::Double,
            RatatuiBorderType::Thick => Self::Thick,
            RatatuiBorderType::QuadrantInside => Self::QuadrantInside,
            RatatuiBorderType::QuadrantOutside => Self::QuadrantOutside,
        }
    }
}

impl Into<RatatuiBorderType> for BorderType {
    fn into(self) -> RatatuiBorderType {
        match self {
            BorderType::Plain => RatatuiBorderType::Plain,
            BorderType::Rounded => RatatuiBorderType::Rounded,
            BorderType::Double => RatatuiBorderType::Double,
            BorderType::Thick => RatatuiBorderType::Thick,
            BorderType::QuadrantInside => RatatuiBorderType::QuadrantInside,
            BorderType::QuadrantOutside => RatatuiBorderType::QuadrantOutside,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    styles: HashMap<String, FetchingStyle>,
    bordered: bool,
    #[serde(default)]
    border_type: BorderType,
    title: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            styles: HashMap::new(),
            bordered: false,
            border_type: BorderType::Rounded,
            title: None,
        }
    }
}
