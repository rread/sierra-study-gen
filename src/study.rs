use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Study {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub region: i8,
    #[serde(default)]
    pub autoloop: bool,
    #[serde(default)]
    pub enable_extra_data: bool,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
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
}

#[derive(Serialize, Deserialize)]
pub struct Output {
    pub label: String,
    pub name: String,
    pub color: String,
    pub width: i8,
    pub style: String,
}

impl Output {
    // pub fn new(label: String, name: String) -> Self {
    //     Self {
    //         label,
    //         name,
    //         color: "RGB(0, 255, 0)".to_string(),
    //         width: 1,
    //         style: "Line".to_string(),
    //     }
    // }

    pub fn var_name(&self) -> String {
        format!("sg_{}", self.label)
    }

    pub fn enum_name(&self) -> String {
        format!("SG_{}_IDX", self.label.to_ascii_uppercase())
    }

    pub fn sc_style(&self) -> String {
        if self.name.is_empty() {
            return String::from("DRAWSTYLE_IGNORE");
        }

        match self.style.as_str() {
            "Line" => String::from("DRAWSTYLE_LINE"),
            "Bar" => String::from("DRAWSTYLE_BAR"),
            "Ignore" | "" | _ => String::from("DRAWSTYLE_IGNORE"),
        }
    }
}
