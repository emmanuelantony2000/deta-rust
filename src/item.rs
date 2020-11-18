use std::string;

use serde::{Deserialize, Serialize};

/// An item which is sent or retrieved from Deta Base.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct Item<T> {
    pub key: Option<String>,
    pub value: T,
}

impl<T> Item<T> {
    /// Make a new item with a value.
    ///
    /// # Examples
    ///
    /// ```
    /// use deta::Item;
    ///
    /// let item = Item::new(5);
    /// assert_eq!(item.key, None);
    /// assert_eq!(item.value, 5);
    /// ```
    pub fn new(value: T) -> Self {
        Self { key: None, value }
    }

    /// Make a new item with a key and a value.
    ///
    /// # Examples
    ///
    /// ```
    /// use deta::Item;
    ///
    /// let item = Item::new_with_key("five", 5);
    /// assert_eq!(item.key.unwrap_or_default(), "five");
    /// assert_eq!(item.value, 5);
    /// ```
    pub fn new_with_key(key: impl string::ToString, value: T) -> Self {
        Self {
            key: Some(key.to_string()),
            value,
        }
    }
}
