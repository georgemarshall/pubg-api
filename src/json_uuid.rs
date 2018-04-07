use std::fmt;

use json_api::value::Stringify;
use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use uuid;

/// Wrapper type to satisfy the `Stringify` trait requirement
pub struct Uuid {
    inner: uuid::Uuid,
}

impl fmt::Debug for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<'de> Deserialize<'de> for Uuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        uuid::Uuid::deserialize(deserializer).map(|inner| Uuid { inner })
    }
}

impl Serialize for Uuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl Stringify for Uuid {
    fn to_bytes(&self) -> Vec<u8> {
        self.inner.as_bytes().to_vec()
    }

    fn stringify(&self) -> String {
        self.inner.hyphenated().to_string()
    }
}
