pub struct Study {
    pub name: String,
    pub description: String,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

pub struct Input {
    pub label: String,
    pub name: String,
    pub description: String,
    pub sctype: InputType,
}

impl Input {
    pub fn var_name(&self) -> String {
        format!("in_{}", self.label)
    }

    pub fn enum_name(&self) -> String {
        format!("IN_{}_IDX", self.label.to_ascii_uppercase())
    }
}
pub enum InputType {
    Int(i32),
    Float(f32),
    Bool(bool),
    Color(String),
    MovingAvg(String),
    Selection(String),
}

pub struct Output {
    pub label: String,
    pub name: String,
    pub color: String,
    pub width: i8,
    pub style: String,
}

impl Output {
    pub fn new(label: String, name: String) -> Self {
        Self {
            label,
            name,
            color: "RGB(0, 255, 0)".to_string(),
            width: 1,
            style: "Line".to_string(),
        }
    }

    pub fn var_name(&self) -> String {
        format!("sg_{}", self.label)
    }

    pub fn enum_name(&self) -> String {
        format!("SG_{}_IDX", self.label.to_ascii_uppercase())
    }

    pub fn sc_style(&self) -> String {
        if self.name.is_empty() {
            String::from("DRAWSTYLE_IGNORE")
        } else {
            String::from("DRAWSTYLE_LINE")
        }
    }
}
