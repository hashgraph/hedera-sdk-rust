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

use hedera_proto::services;
use hedera_proto::services::schedule_service_client::ScheduleServiceClient;
use tonic::transport::Channel;

use crate::query::{
    AnyQueryData,
    QueryExecute,
    ToQueryProtobuf,
};
use crate::{
    BoxGrpcFuture,
    Error,
    LedgerId,
    Query,
    ScheduleId,
    ScheduleInfo,
    ToProtobuf,
    ValidateChecksums,
};

/// Get all the information about a schedule.
pub type ScheduleInfoQuery = Query<ScheduleInfoQueryData>;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct ScheduleInfoQueryData {
    schedule_id: Option<ScheduleId>,
}

impl From<ScheduleInfoQueryData> for AnyQueryData {
    #[inline]
    fn from(data: ScheduleInfoQueryData) -> Self {
        Self::ScheduleInfo(data)
    }
}

impl ScheduleInfoQuery {
    /// Returns the schedule ID for which information is requested.
    #[must_use]
    pub fn get_schedule_id(&self) -> Option<ScheduleId> {
        self.data.schedule_id
    }

    /// Sets the schedule ID for which information is requested.
    pub fn schedule_id(&mut self, id: impl Into<ScheduleId>) -> &mut Self {
        self.data.schedule_id = Some(id.into());
        self
    }
}

impl ToQueryProtobuf for ScheduleInfoQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let schedule_id = self.schedule_id.to_protobuf();

        services::Query {
            query: Some(services::query::Query::ScheduleGetInfo(services::ScheduleGetInfoQuery {
                schedule_id,
                header: Some(header),
            })),
        }
    }
}

impl QueryExecute for ScheduleInfoQueryData {
    type Response = ScheduleInfo;

    fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> BoxGrpcFuture<'_, services::Response> {
        Box::pin(async { ScheduleServiceClient::new(channel).get_schedule_info(request).await })
    }
}

impl ValidateChecksums for ScheduleInfoQueryData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.schedule_id.validate_checksums(ledger_id)
    }
}
