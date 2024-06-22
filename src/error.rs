use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),
}
