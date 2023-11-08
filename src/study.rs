use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Serialize, Deserialize)]
pub struct Study {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub region: i8,
    #[serde(default = "default_true")]
    pub autoloop: bool,
    #[serde(default = "default_false")]
    pub enable_extra_data: bool,
    #[serde(default = "default_false")]
    pub pointer_events: bool,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub private_class: Option<String>,
}

fn default_false() -> bool {
    false
}

fn default_true() -> bool {
    true
}
impl Study {
    pub fn class_name(&self) -> String {
        self.name.chars().filter(|c| !c.is_whitespace()).collect()
    }

    pub fn def_name(&self) -> String {
        let result = str::replace(&*self.name, " ", "_");
        result.to_ascii_uppercase()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub label: String,
    pub name: String,
    pub description: String,
    pub intype: InputType,
}

impl Input {
    pub fn var_name(&self) -> String {
        format!("in_{}", self.label)
    }

    pub fn enum_name(&self) -> String {
        format!("IN_{}_IDX", self.label.to_ascii_uppercase())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
// #[serde(tag = "type", content = "args")]
pub enum InputType {
    Int(i32),
    Float(f32),
    Bool(bool),
    Color(String),
    MovingAvg(String),
    Selection(String),
    Data(String),
}

#[derive(Serialize, Deserialize)]
pub struct Output {
    pub label: String,
    #[serde(default)]
    pub name: String,
    #[serde(default = "default_color")]
    pub color: String,
    pub second_color: Option<String>,
    #[serde(default = "default_width")]
    pub width: i8,
    #[serde(default = "default_drawstyle")]
    pub style: DrawStyle,
    pub auto_color: Option<AutoColorType>,
}

fn default_color() -> String {
    "RGB(255,255,255)".to_string()
}

fn default_width() -> i8 {
    1
}

fn default_drawstyle() -> DrawStyle {
    DrawStyle::Ignore
}
#[derive(Serialize, Deserialize)]
pub enum AutoColorType {
    None,
    Gradient,
    BaseGraph,
    Slope,
}

impl fmt::Display for AutoColorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AutoColorType::None => write!(f, "AUTOCOLOR_NONE"),
            AutoColorType::Gradient => write!(f, "AUTOCOLOR_GRADIENT"),
            AutoColorType::BaseGraph => write!(f, "AUTOCOLOR_BASEGRAPH"),
            AutoColorType::Slope => write!(f, "AUTOCOLOR_SLOPE"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum DrawStyle {
    Ignore,
    Line,
    Bar,
    Text,
    Background,
    TransparentBackground,
    CandleFill,
}
impl fmt::Display for DrawStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DrawStyle::Ignore => write!(f, "DRAWSTYLE_IGNORE"),
            DrawStyle::Line => write!(f, "DRAWSTYLE_LINE"),
            DrawStyle::Bar => write!(f, "DRAWSTYLE_BAR"),
            DrawStyle::Text => write!(f, "DRAWSTYLE_TEXT"),
            DrawStyle::Background => write!(f, "DRAWSTYLE_BACKGROUND"),
            DrawStyle::TransparentBackground => write!(f, "DRAWSTYLE_BACKGROUND_TRANSPARENT"),
            DrawStyle::CandleFill => write!(f, "DRAWSTYLE_COLOR_BAR_CANDLE_FILL"),
        }
    }
}

impl Output {
    pub fn var_name(&self) -> String {
        format!("sg_{}", self.label)
    }

    pub fn enum_name(&self) -> String {
        format!("SG_{}_IDX", self.label.to_ascii_uppercase())
    }
}
