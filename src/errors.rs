pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("HTTP request failed")]
    HTTPRequestError(#[from] reqwest::Error),

    #[error("Registration failed")]
    RegisterFailed,

    #[error("Registration failed")]
    LoginFailed,

    #[error("Could not [de]serialize data")]
    SerializationError(#[from] serde_json::Error),

    #[error("No character currently selected")]
    NoCharacter,
}
