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

pub trait ToProtobuf: Send + Sync {
    type Protobuf;

    fn to_protobuf(&self) -> Self::Protobuf;
}

pub trait FromProtobuf<Protobuf> {
    fn from_protobuf(pb: Protobuf) -> crate::Result<Self>
    where
        Self: Sized;
}

impl<T, P> FromProtobuf<Vec<P>> for Vec<T>
where
    T: FromProtobuf<P>,
{
    fn from_protobuf(pb: Vec<P>) -> crate::Result<Self>
    where
        Self: Sized,
    {
        pb.into_iter().map(T::from_protobuf).collect::<crate::Result<Vec<_>>>()
    }
}
