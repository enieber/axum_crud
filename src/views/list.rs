use crate::views::form::FormField;

pub struct ListAction {
    pub label: String,
    pub url: String,
    pub method: String,
    pub is_danger: bool,
}

impl ListAction {
    pub fn edit(id: &str, page_name: &str) -> Self {
        Self {
            label: "Edit".into(),
            url: format!("/{}/{}/edit", page_name, id),
            method: "get".into(),
            is_danger: false,
        }
    }

    pub fn delete(id: &str, page_name: &str) -> Self {
        Self {
            label: "Excluir".into(),
            url: format!("/{}/{}/delete", page_name, id),
            method: "post".into(),
            is_danger: true,
        }
    }
}

pub struct ListItemView {
    pub id: String,
    pub fields: Vec<FormField>,
    pub actions: Vec<ListAction>,
}

impl ListItemView {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            fields: vec![],
            actions: vec![],
        }
    }

    pub fn with_fields(mut self, fields: Vec<FormField>) -> Self {
        self.fields = fields;
        self
    }

    pub fn with_actions(mut self, actions: Vec<ListAction>) -> Self {
        self.actions = actions;
        self
    }
}
