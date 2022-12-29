use hedera_proto::services;

use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};
use crate::Key;

// note: it appears keylists "just" implement the APIs of arrays in their language, which means, uh...
// todo: Copy over the _entire_ `Vec` API?.
/// A list of keys with an optional threshold.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct KeyList {
    // todo: better doc comment?
    /// The list of keys.
    #[cfg_attr(feature = "ffi", serde(skip_serializing_if = "Vec::is_empty", default))]
    pub keys: Vec<Key>,

    #[cfg_attr(feature = "ffi", serde(skip_serializing_if = "Option::is_none", default))]
    /// If [`Some`]: The minimum number of keys that must sign.
    pub threshold: Option<u32>,
}

impl std::ops::Deref for KeyList {
    type Target = Vec<Key>;

    fn deref(&self) -> &Self::Target {
        &self.keys
    }
}

impl KeyList {
    /// Create a new empty key list.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` if this keylist is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// Removes and returns the element at position index within the key list, shifting all elements after it to the left.
    ///
    /// # Panics
    /// Panics if index is out of bounds.
    pub fn remove(&mut self, index: usize) -> Key {
        self.keys.remove(index)
    }

    // why not `ToProtobuf`? because `ToProtobuf` should return a `KeyList`.
    pub(crate) fn to_protobuf_key(&self) -> services::key::Key {
        let key_list = services::KeyList { keys: self.keys.to_protobuf() };

        if let Some(threshold) = self.threshold {
            return services::key::Key::ThresholdKey(services::ThresholdKey {
                threshold,
                keys: Some(key_list),
            });
        };

        services::key::Key::KeyList(key_list)
    }
}

impl ToProtobuf for KeyList {
    type Protobuf = services::KeyList;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::KeyList { keys: self.keys.to_protobuf() }
    }
}

impl FromIterator<Key> for KeyList {
    fn from_iter<T: IntoIterator<Item = Key>>(iter: T) -> Self {
        Self { keys: iter.into_iter().collect(), threshold: None }
    }
}

impl From<Vec<Key>> for KeyList {
    fn from(value: Vec<Key>) -> Self {
        Self { keys: value, threshold: None }
    }
}

impl FromProtobuf<services::KeyList> for KeyList {
    fn from_protobuf(pb: services::KeyList) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Vec::from_protobuf(pb.keys).map(Self::from)
    }
}

impl FromProtobuf<services::ThresholdKey> for KeyList {
    fn from_protobuf(pb: services::ThresholdKey) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let keys = Vec::from_protobuf(pb.keys.unwrap_or_default().keys)?;
        Ok(Self { keys, threshold: Some(pb.threshold) })
    }
}
