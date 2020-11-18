use std::fmt;
use std::sync::Arc;

use reqwest::{header, Client};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub use error::{Error, Result};
pub use item::Item;
pub use update::Update;

mod error;
mod item;
mod update;

const URL: &str = "https://database.deta.sh/v1/";

/// The `Deta` client.
///
/// This uses `reqwest::Client` internally. Create one and reuse it.
///
/// You don't need to wrap it with a `Rc` or an `Arc`, because it uses an `Arc` internally.
/// To reuse the client or pass it on to another thread, `.clone()` it.
#[derive(Clone)]
pub struct Deta {
    client: Client,
    url: Arc<String>,
    key: Arc<String>,
    base_name: Option<Arc<String>>,
}

impl Deta {
    /// Creates a new client.
    ///
    /// Use this if you have the `Project Key` in the env var `DETA_PROJECT_KEY`.
    ///
    /// # Errors
    ///
    /// * [`Error::KeyNotFound`](crate::Error::KeyNotFound)
    /// * [`Error::InvalidKey`](crate::Error::InvalidKey)
    /// * [`Error::ClientInitError`](crate::Error::ClientInitError)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use deta::Deta;
    /// # fn main() -> deta::Result<()> {
    /// let deta = Deta::new()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Result<Self> {
        use std::env;

        let key = env::var("DETA_PROJECT_KEY").map_err(|_| Error::KeyNotFound)?;
        Self::key(key)
    }

    /// Creates a new client.
    ///
    /// # Arguments
    ///
    /// * `key`: The `Project Key`.
    ///
    /// # Errors
    ///
    /// * [`Error::InvalidKey`](crate::Error::InvalidKey)
    /// * [`Error::ClientInitError`](crate::Error::ClientInitError)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use deta::Deta;
    /// # fn main() -> deta::Result<()> {
    /// let key = "some key";
    /// let deta = Deta::new_with_key(key)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_with_key(key: impl AsRef<str>) -> Result<Self> {
        let key = key.as_ref().to_string();
        Self::key(key)
    }

    fn key(key: String) -> Result<Self> {
        let valid = !key.contains(|c: char| {
            !(c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '-' || c == '~')
        });

        if !valid {
            return Err(Error::InvalidKey);
        }

        let pid = key.split('_').next().ok_or(Error::InvalidKey)?;

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::HeaderName::from_static("x-api-key"),
            header::HeaderValue::from_str(&key).map_err(|_| Error::InvalidKey)?,
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|_| Error::ClientInitError)?;
        let url = Arc::new(format!("{}{}", URL, pid));
        let key = Arc::new(key);

        Ok(Self {
            client,
            url,
            key,
            base_name: None,
        })
    }

    /// Sets the name of the database for the client.
    ///
    /// This internally clones the client and sets the name of the database.
    ///
    /// # Arguments
    ///
    /// * `base_name`: The name of the database.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use deta::Deta;
    /// # fn main() -> deta::Result<()> {
    /// let deta = Deta::new()?;
    ///
    /// let db_main = deta.base("main");
    /// let db_test = deta.base("test");
    /// # Ok(())
    /// # }
    /// ```
    pub fn base(&self, base_name: impl AsRef<str>) -> Self {
        Self {
            base_name: Some(Arc::new(base_name.as_ref().to_string())),
            ..self.clone()
        }
    }

    /// Get a stored item.
    ///
    /// # Arguments
    ///
    /// * `key`: The key (aka. ID) of the item you want to retrieve.
    ///
    /// # Errors
    ///
    /// * [`Error::BaseNameNotPresent`](crate::Error::BaseNameNotPresent)
    /// * [`Error::RequestSendError`](crate::Error::RequestSendError)
    /// * [`Error::ItemNotFound`](crate::Error::ItemNotFound)
    /// * [`Error::JSONDeserializingFailed`](crate::Error::JSONDeserializingFailed)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use deta::{Deta, Item};
    /// # #[tokio::main]
    /// # async fn main() -> deta::Result<()> {
    /// let deta = Deta::new()?;
    ///
    /// let base = deta.base("main");
    /// let item: Item<usize> = base.get("id").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get<T>(&self, key: impl fmt::Display) -> Result<Item<T>>
    where
        T: DeserializeOwned,
    {
        let url = format!(
            "{}/{}/items/{}",
            self.url,
            self.base_name.as_ref().ok_or(Error::BaseNameNotPresent)?,
            key
        );

        self.client
            .get(&url)
            .send()
            .await
            .map_err(|_| Error::RequestSendError)?
            .error_for_status()
            .map_err(|_| Error::ItemNotFound)?
            .json()
            .await
            .map_err(|_| Error::JSONDeserializingFailed)
    }

    /// Delete a stored item.
    ///
    /// # Arguments
    ///
    /// * `key`: The key (aka. ID) of the item you want to delete.
    ///
    /// # Errors
    ///
    /// * [`Error::BaseNameNotPresent`](crate::Error::BaseNameNotPresent)
    /// * [`Error::RequestSendError`](crate::Error::RequestSendError)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use deta::Deta;
    /// # #[tokio::main]
    /// # async fn main() -> deta::Result<()> {
    /// let deta = Deta::new()?;
    ///
    /// let base = deta.base("main");
    /// base.delete("id").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, key: impl fmt::Display) -> Result<()> {
        let url = format!(
            "{}/{}/items/{}",
            self.url,
            self.base_name.as_ref().ok_or(Error::BaseNameNotPresent)?,
            key
        );

        self.client
            .delete(&url)
            .send()
            .await
            .map_err(|_| Error::RequestSendError)?;

        Ok(())
    }

    /// Stores multiple items in a single request.
    /// This request overwrites an item if the key already exists.
    ///
    /// It returns a tuple of both processed and failed items.
    ///
    /// # Arguments
    ///
    /// * `items`: A `Vec` of `Item`s.
    ///
    /// # Errors
    ///
    /// * [`Error::BaseNameNotPresent`](crate::Error::BaseNameNotPresent)
    /// * [`Error::RequestSendError`](crate::Error::RequestSendError)
    /// * [`Error::BadRequest`](crate::Error::BadRequest)
    /// * [`Error::JSONDeserializingFailed`](crate::Error::JSONDeserializingFailed)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use deta::{Deta, Item};
    /// # #[tokio::main]
    /// # async fn main() -> deta::Result<()> {
    /// let deta = Deta::new()?;
    ///
    /// let base = deta.base("main");
    /// let numbers = ["zero", "one", "two", "three", "four"];
    /// let vec = numbers
    ///     .iter()
    ///     .enumerate()
    ///     .map(|(c, x)| Item::new_with_key(c, x))
    ///     .collect::<Vec<_>>();
    /// let (processed, failed): (Vec<Item<String>>, Vec<Item<String>>) = base.put(vec).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn put<T, U>(&self, items: Vec<Item<T>>) -> Result<(Vec<Item<U>>, Vec<Item<U>>)>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let url = format!(
            "{}/{}/items",
            self.url,
            self.base_name.as_ref().ok_or(Error::BaseNameNotPresent)?,
        );

        let req_body = Put { items };

        let PutResult { processed, failed }: PutResult<U> = self
            .client
            .put(&url)
            .json(&req_body)
            .send()
            .await
            .map_err(|_| Error::RequestSendError)?
            .error_for_status()
            .map_err(|_| Error::BadRequest)?
            .json()
            .await
            .map_err(|_| Error::JSONDeserializingFailed)?;

        let Put { items: processed } = processed;
        let Put { items: failed } = failed;

        Ok((processed, failed))
    }

    /// Creates a new item only if no item with the same `key` exists.
    ///
    /// Returns the key, if successful. If the same key exists returns an Error.
    ///
    /// # Arguments
    ///
    /// * `item`: An `Item`.
    ///
    /// # Errors
    ///
    /// * [`Error::BaseNameNotPresent`](crate::Error::BaseNameNotPresent)
    /// * [`Error::RequestSendError`](crate::Error::RequestSendError)
    /// * [`Error::KeyConflict`](crate::Error::KeyConflict)
    /// * [`Error::BadRequest`](crate::Error::BadRequest)
    /// * [`Error::ServerError`](crate::Error::ServerError)
    /// * [`Error::JSONDeserializingFailed`](crate::Error::JSONDeserializingFailed)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use deta::{Deta, Item};
    /// # #[tokio::main]
    /// # async fn main() -> deta::Result<()> {
    /// let deta = Deta::new()?;
    ///
    /// let base = deta.base("main");
    /// let item = Item::new_with_key(5, "five");
    /// base.insert(item).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn insert<T>(&self, item: Item<T>) -> Result<String>
    where
        T: Serialize,
    {
        let url = format!(
            "{}/{}/items",
            self.url,
            self.base_name.as_ref().ok_or(Error::BaseNameNotPresent)?,
        );

        let req_body = Insert { item };

        let json: serde_json::Value = self
            .client
            .post(&url)
            .json(&req_body)
            .send()
            .await
            .map_err(|_| Error::RequestSendError)?
            .error_for_status()
            .map_err(|e| {
                if let Some(x) = e.status() {
                    if x == reqwest::StatusCode::CONFLICT {
                        return Error::KeyConflict;
                    } else if x == reqwest::StatusCode::BAD_REQUEST {
                        return Error::BadRequest;
                    }
                }
                Error::ServerError
            })?
            .json()
            .await
            .map_err(|_| Error::JSONDeserializingFailed)?;

        Ok(json["key"].as_str().ok_or(Error::ServerError)?.to_string())
    }

    /// Updates an item only if an item with `key` exists.
    ///
    /// # Arguments
    ///
    /// * `update`: An `Update` struct.
    ///
    /// # Errors
    ///
    /// * [`Error::BaseNameNotPresent`](crate::Error::BaseNameNotPresent)
    /// * [`Error::RequestSendError`](crate::Error::RequestSendError)
    /// * [`Error::KeyNonexistent`](crate::Error::KeyNonexistent)
    /// * [`Error::BadRequest`](crate::Error::BadRequest)
    /// * [`Error::ServerError`](crate::Error::ServerError)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use deta::{Deta, Update};
    /// # #[tokio::main]
    /// # async fn main() -> deta::Result<()> {
    /// let deta = Deta::new()?;
    ///
    /// let base = deta.base("main");
    /// deta.update(
    ///     "user-a",
    ///     Update::new()
    ///             .set("profile.age", 33)
    ///         .set("profile.active", true)
    ///         .set("profile.email", "jimmy@deta.sh")
    ///         .increment("purchases", 2)
    ///         .append("likes", "ramen")
    ///         .prepend("likes", "noodles")
    ///         .delete("profile.hometown")
    ///         .delete("on_mobile"),
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(&self, key: impl fmt::Display, update: Update) -> Result<()> {
        let url = format!(
            "{}/{}/items/{}",
            self.url,
            self.base_name.as_ref().ok_or(Error::BaseNameNotPresent)?,
            key
        );

        self.client
            .patch(&url)
            .json(&update)
            .send()
            .await
            .map_err(|_| Error::RequestSendError)?
            .error_for_status()
            .map_err(|e| {
                if let Some(x) = e.status() {
                    if x == reqwest::StatusCode::NOT_FOUND {
                        return Error::KeyNonexistent;
                    } else if x == reqwest::StatusCode::BAD_REQUEST {
                        return Error::BadRequest;
                    }
                }
                Error::ServerError
            })?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct Insert<T> {
    item: Item<T>,
}

#[derive(Serialize, Deserialize)]
struct Put<T> {
    items: Vec<Item<T>>,
}

#[derive(Serialize, Deserialize)]
struct PutResult<T> {
    processed: Put<T>,
    failed: Put<T>,
}
