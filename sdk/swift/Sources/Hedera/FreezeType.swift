/*
 * ‌
 * Hedera Swift SDK
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

public enum FreezeType: Codable {
    /// An (invalid) default value for this enum, to ensure the client explicitly sets
    /// the intended type of freeze transaction.
    case unknown

    /// Freezes the network at the specified time. The start_time field must be provided and
    /// must reference a future time. Any values specified for the update_file and file_hash
    /// fields will be ignored. This transaction does not perform any network changes or
    /// upgrades and requires manual intervention to restart the network.
    case freezeOnly

    /// A non-freezing operation that initiates network wide preparation in advance of a
    /// scheduled freeze upgrade. The update_file and file_hash fields must be provided and
    /// valid. The start_time field may be omitted and any value present will be ignored.
    case prepareUpgrade

    /// Freezes the network at the specified time and performs the previously prepared
    /// automatic upgrade across the entire network.
    case freezeUpgrade

    /// Aborts a pending network freeze operation.
    case freezeAbort

    /// Performs an immediate upgrade on auxilary services and containers providing
    /// telemetry/metrics. Does not impact network operations.
    case telemetryUpgrade
}
