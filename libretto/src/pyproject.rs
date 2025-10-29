use std::collections::{HashMap};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PyProjectToml {
    #[serde(flatten)]
    inner: pyproject_toml::PyProjectToml,
    pub tool: Option<Tool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Tool {
    pub libretto: Option<ToolLibretto>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ToolLibretto {
    pub venv: Option<String>,
    pub uv: Option<bool>,
    pub tasks: HashMap<String, TaskValue>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TaskValue {
    String(String),
    Array(Vec<CommandElement>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum CommandElement {
    CommandObject(CommandObject),
    SingleString(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandObject {
    pub cmd: String,
    pub platforms: Vec<String>, 
}

impl std::ops::Deref for PyProjectToml {
    type Target = pyproject_toml::PyProjectToml;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl PyProjectToml {
    pub fn new(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }
}