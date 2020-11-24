use thiserror::Error;

/// Errors that may occur.
#[derive(Error, Debug)]
pub enum Error {
    /// Key not found in environment.
    ///
    /// Ensure that the key is stored in the `DETA_PROJECT_KEY` environment variable.
    #[error("key not found in env")]
    KeyNotFound,

    /// The key supplied is invalid.
    ///
    /// Ensure that you have a valid key.
    #[error("invalid key")]
    InvalidKey,

    /// Error occurred while initializing client.
    ///
    /// This occurs when the TLS backend cannot be initialized,
    /// or the resolver cannot load the system configuration.
    #[error("error while initializing client")]
    ClientInitError,

    /// Base name not present.
    ///
    /// You might have initialized the Deta client, but not assigned it to a Deta Base.
    #[error("base name not present")]
    BaseNameNotPresent,

    /// The length of the vector is too long.
    ///
    /// There can't be more than 25 items at a time.
    #[error("vec length too long")]
    VecTooLong,

    /// Error occurred while sending a request.
    ///
    /// Check your network connectivity.
    #[error("error while sending request")]
    RequestSendError,

    /// Item not found in the Deta Base.
    #[error("item not found")]
    ItemNotFound,

    /// Key already exists in the Deta Base.
    #[error("key already exists")]
    KeyConflict,

    /// Key doesn't exist in the Deta Base.
    #[error("key doesn't exist")]
    KeyNonexistent,

    /// Bad Request.
    ///
    /// Occurs in the following cases:
    /// * If the number of items in the requests exceeds 25
    /// * If total request size exceeds 16 MB
    /// * If any individual item exceeds 400KB
    /// * If there are two items with identical keys
    #[error("bad request")]
    BadRequest,

    /// Server error.
    ///
    /// The server didn't return the expected response.
    #[error("server error")]
    ServerError,

    /// Serializing to JSON failed.
    ///
    /// The request didn't successfully serialize to JSON.
    #[error("JSON serializing failed")]
    JSONSerializingFailed,

    /// Deserializing to JSON failed.
    ///
    /// The response didn't successfully deserialize to JSON.
    #[error("JSON deserializing failed")]
    JSONDeserializingFailed,
}

/// A `Result` alias where the `Err` case is `deta::Error`.
pub type Result<T> = std::result::Result<T, Error>;
