use wasm_bindgen::JsValue;

use thiserror::Error as ThisError;

use std::result::Result as StdResult;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Unknown JavaScript Error")]
    UnknownJs(JsValue),

    #[error("Failed to communicate with remote.")]
    Networking(#[from] reqwest::Error),

    #[error("Failed to parse Url.")]
    Url(#[from] url::ParseError),

    #[error("The manifest is not valid.")]
    InvalidManifest(#[from] toml::de::Error),
}

impl From<JsValue> for Error {
    fn from(e: JsValue) -> Error {
        Error::UnknownJs(e)
    }
}

impl From<Error> for JsValue {
    fn from(e: Error) -> JsValue {
        match e {
            Error::UnknownJs(val) => val,
            Error::Networking(e) => e.to_string().into(),
            Error::Url(e) => e.to_string().into(),
            Error::InvalidManifest(e) => e.to_string().into(),
        }
    }
}

pub type Result<T> = StdResult<T, Error>;
