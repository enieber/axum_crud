use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum FieldKind {
    Text,
    Textarea,
    Checkbox,
    Number,
}

#[derive(Debug, Clone)]
pub struct FormField {
    pub name: String,
    pub label: String,
    pub value: String,
    pub kind: FieldKind,
    // pub kind: &'static str,   // "text" | "textarea" | "checkbox" | "number"
    pub required: bool,
    pub placeholder: Option<String>,
    pub error: Option<String>,
}

impl FormField {
    pub fn text(name: &str, label: &str, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            value: value.into(),
            kind: FieldKind::Text,
            required: false,
            placeholder: None,
            error: None,
        }
    }

    pub fn checkbox(name: &str, label: &str, checked: bool) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            value: if checked { "true".into() } else { String::new() },
            kind: FieldKind::Checkbox,
            required: false,
            placeholder: None,
            error: None,
        }
    }

    pub fn required(mut self) -> Self { self.required = true; self }
    pub fn placeholder(mut self, p: &str) -> Self { self.placeholder = Some(p.into()); self }
    pub fn error(mut self, e: &str) -> Self { self.error = Some(e.into()); self }
}
