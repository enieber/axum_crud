use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum FieldKind {
    Text,
    Password,
    Textarea,
    Checkbox,
    Number,
    Badge,
    Image,
    Link,
    Date,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct FormField {
    pub name: String,
    pub label: String,
    pub value: String,
    pub kind: FieldKind,
    pub required: bool,
    pub placeholder: Option<String>,
    pub error: Option<String>,
    pub url: Option<String>,
}

impl FormField {
    pub fn password(name: &str, label: &str, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            value: value.into(),
            kind: FieldKind::Password,
            required: false,
            placeholder: None,
            error: None,
            url: None,
        }
    }
    pub fn textarea(name: &str, label: &str, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            value: value.into(),
            kind: FieldKind::Textarea,
            required: false,
            placeholder: None,
            error: None,
            url: None,
        }
    }

    pub fn text(name: &str, label: &str, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            value: value.into(),
            kind: FieldKind::Text,
            required: false,
            placeholder: None,
            error: None,
            url: None,
        }
    }

    pub fn checkbox(name: &str, label: &str, checked: bool) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            value: if checked {
                "true".into()
            } else {
                String::new()
            },
            kind: FieldKind::Checkbox,
            required: false,
            placeholder: None,
            error: None,
            url: None,
        }
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    pub fn placeholder(mut self, p: &str) -> Self {
        self.placeholder = Some(p.into());
        self
    }
    pub fn error(mut self, e: &str) -> Self {
        self.error = Some(e.into());
        self
    }

    pub fn badge(name: &str, label: &str, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            value: value.into(),
            kind: FieldKind::Badge,
            required: false,
            placeholder: None,
            error: None,
            url: None,
        }
    }

    pub fn image(name: &str, label: &str, url: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            value: String::new(),
            kind: FieldKind::Image,
            required: false,
            placeholder: None,
            error: None,
            url: Some(url.into()),
        }
    }

    pub fn link(name: &str, label: &str, text: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            value: text.into(),
            kind: FieldKind::Link,
            required: false,
            placeholder: None,
            error: None,
            url: Some(url.into()),
        }
    }

    pub fn date(name: &str, label: &str, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            value: value.into(),
            kind: FieldKind::Date,
            required: false,
            placeholder: None,
            error: None,
            url: None,
        }
    }
}
