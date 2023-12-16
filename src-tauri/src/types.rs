use serde::{ser::Serializer, Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseItem<T> {
    Single(T),
    Multiple(Vec<T>),
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KittyResponse<T> {
    pub data: Option<ResponseItem<T>>,
    pub code: i8,
    pub msg: Option<String>,
}

impl<T> KittyResponse<T> {
    pub fn new(code: i8, data: ResponseItem<T>, msg: &str) -> Self {
        Self {
            code,
            data: Some(data),
            msg: Some(msg.to_string()),
        }
    }

    pub fn from_msg(code: i8, msg: &str) -> Self {
        Self {
            data: None,
            code,
            msg: Some(msg.to_string()),
        }
    }
}

impl<T> Default for KittyResponse<T> {
    fn default() -> Self {
        Self {
            data: None,
            code: Default::default(),
            msg: None,
        }
    }
}

// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
pub enum KittyCommandError {
    #[error(transparent)]
    RusqliteError(#[from] rusqlite::Error),

    #[error(transparent)]
    DBError(#[from] sea_orm::DbErr),

    #[error(transparent)]
    TauriError(#[from] tauri::Error),

    #[error(transparent)]
    AnyHowError(#[from] anyhow::Error),

    #[error(transparent)]
    TauriShellError(#[from] tauri_plugin_shell::Error),
}

// we must manually implement serde::Serialize
impl Serialize for KittyCommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type CommandResult<T, E = KittyCommandError> = anyhow::Result<T, E>;