use std::collections::HashMap;
use std::string;

use serde::{Deserialize, Serialize};

/// Only used for update requests.
///
/// # Examples
///
/// ```
/// use deta::Update;
///
/// let update = Update::new()
///     .set("profile.age", 33)
///     .set("profile.active", true)
///     .set("profile.email", "jimmy@deta.sh")
///     .increment("purchases", 2)
///     .append("likes", "ramen")
///     .prepend("likes", "noodles")
///     .delete("profile.hometown")
///     .delete("on_mobile");
/// ```
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct Update {
    set: HashMap<String, String>,
    increment: HashMap<String, String>,
    append: HashMap<String, Vec<String>>,
    prepend: HashMap<String, Vec<String>>,
    delete: Vec<String>,
}

impl Update {
    /// To initialize a new empty `Update` struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use deta::Update;
    /// let update = Update::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// To set a new value for an attribute.
    ///
    /// # Examples
    ///
    /// ```
    /// use deta::Update;
    /// let update = Update::new().set("name", "Jimmy");
    /// ```
    pub fn set(mut self, key: impl string::ToString, value: impl string::ToString) -> Self {
        self.set.insert(key.to_string(), value.to_string());
        self
    }

    /// To increment a value for an attribute.
    ///
    /// # Examples
    ///
    /// ```
    /// use deta::Update;
    /// let update = Update::new().increment("age", 1);
    /// ```
    pub fn increment(mut self, key: impl string::ToString, value: impl string::ToString) -> Self {
        self.increment.insert(key.to_string(), value.to_string());
        self
    }

    /// To append a value to the existing value of an attribute.
    ///
    /// # Examples
    ///
    /// ```
    /// use deta::Update;
    /// let update = Update::new().append("likes", "ramen");
    /// ```
    pub fn append(mut self, key: impl string::ToString, value: impl string::ToString) -> Self {
        self.append
            .entry(key.to_string())
            .or_insert_with(Vec::new)
            .push(value.to_string());
        self
    }

    /// To prepend a value to the existing value of an attribute.
    ///
    /// # Examples
    ///
    /// ```
    /// use deta::Update;
    /// let update = Update::new().append("likes", "noodles");
    /// ```
    pub fn prepend(mut self, key: impl string::ToString, value: impl string::ToString) -> Self {
        self.prepend
            .entry(key.to_string())
            .or_insert_with(Vec::new)
            .push(value.to_string());
        self
    }

    /// To delete an attribute.
    ///
    /// # Examples
    ///
    /// ```
    /// use deta::Update;
    /// let update = Update::new().append("likes", "ramen");
    /// ```
    pub fn delete(mut self, key: impl string::ToString) -> Self {
        self.delete.push(key.to_string());
        self
    }
}
