#[derive(PartialEq, Debug, Clone)]
pub struct SigninResident {
    pub name: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

impl SigninResident {
    pub fn avatar_url(&self) -> String {
        self.avatar_url
            .as_ref()
            .map(|m| m.to_string())
            .unwrap_or_else(|| {
                "https://www.gravatar.com/avatar/55ea921519aa759f6a15711ad4faea7c?s=200&d=mp"
                    .to_string()
            })
    }
}

impl Default for SigninResident {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            display_name: None,
            avatar_url: None,
        }
    }
}
