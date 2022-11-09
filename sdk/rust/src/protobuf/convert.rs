/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

use crate::Error;

/// Convert to a `hedera_protobufs` type.
pub trait ToProtobuf: Send + Sync {
    /// The protobuf output.
    type Protobuf;

    /// Convert from [`self`](Self) to [`Self::Protobuf`].
    fn to_protobuf(&self) -> Self::Protobuf;

    /// Convert [`self`](Self) to a protobuf-encoded [`Vec<u8>`].
    #[must_use]
    fn to_bytes(&self) -> Vec<u8>
    where
        Self::Protobuf: prost::Message,
    {
        use prost::Message as _;
        self.to_protobuf().encode_to_vec()
    }
}

impl<T: ToProtobuf> ToProtobuf for Option<T> {
    type Protobuf = Option<T::Protobuf>;

    fn to_protobuf(&self) -> Self::Protobuf {
        self.as_ref().map(T::to_protobuf)
    }
}

impl<T: ToProtobuf> ToProtobuf for Vec<T> {
    type Protobuf = Vec<T::Protobuf>;

    fn to_protobuf(&self) -> Self::Protobuf {
        self.iter().map(T::to_protobuf).collect()
    }
}

/// Convert from a `hedera_protobufs` type.
pub trait FromProtobuf<Protobuf> {
    /// Attempt to convert from `Protobuf` to `Self`.
    ///
    /// This method is *not* `dyn` safe, and explicitly has a `Self: Sized` bound.
    // todo: errors
    #[allow(clippy::missing_errors_doc)]
    fn from_protobuf(pb: Protobuf) -> crate::Result<Self>
    where
        Self: Sized;

    // fixme(sr): I'm not happy with this doc comment.
    /// Create a new `Self` from protobuf-encoded `bytes`.
    ///
    /// This method is *not* `dyn` safe, and explicitly has a `Self: Sized` bound.
    ///
    /// # Errors
    /// - [`Error::FromProtobuf`] if `Protobuf` fails to decode from the bytes.
    /// - If [`from_protobuf`](Self::from_protobuf) would fail.
    fn from_bytes(bytes: &[u8]) -> crate::Result<Self>
    where
        Self: Sized,
        Protobuf: prost::Message + Default,
    {
        Protobuf::decode(bytes).map_err(Error::from_protobuf).and_then(Self::from_protobuf)
    }
}

impl<T, P> FromProtobuf<Option<P>> for Option<T>
where
    T: FromProtobuf<P>,
{
    fn from_protobuf(pb: Option<P>) -> crate::Result<Self>
    where
        Self: Sized,
    {
        pb.map(T::from_protobuf).transpose()
    }
}

impl<T, P> FromProtobuf<Vec<P>> for Vec<T>
where
    T: FromProtobuf<P>,
{
    fn from_protobuf(pb: Vec<P>) -> crate::Result<Self>
    where
        Self: Sized,
    {
        pb.into_iter().map(T::from_protobuf).collect()
    }
}
