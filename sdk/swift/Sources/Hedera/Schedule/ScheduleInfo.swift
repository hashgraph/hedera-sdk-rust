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

import CHedera
import Foundation
import HederaProtobufs

/// Response from `ScheduleInfoQuery`.
public final class ScheduleInfo: Codable {
    /// The ID of the schedule for which information is requested.
    public let scheduleId: ScheduleId

    /// The account that created the scheduled transaction.
    public let creatorAccountId: AccountId

    /// The account paying for the execution of the scheduled transaction.
    public let payerAccountId: AccountId?

    /// The signatories that have provided signatures so far for the schedule
    /// transaction.
    public let signatories: KeyList

    /// The key which is able to delete the schedule transaction if set.
    public let adminKey: Key?

    /// The transaction id that will be used in the record of the scheduled transaction (if
    /// it executes).
    public let scheduledTransactionId: TransactionId

    private let scheduledTransaction: AnySchedulableTransaction

    /// When set to true, the transaction will be evaluated for execution at `expiration_time`
    /// instead of when all required signatures are received.
    public let waitForExpiry: Bool

    /// Publicly visible information about the Schedule entity.
    public let memo: String

    /// The date and time the schedule transaction will expire
    public let expirationTime: Timestamp?

    /// The time the schedule transaction was executed.
    public let executedAt: Timestamp?

    /// The time the schedule transaction was deleted.
    public let deletedAt: Timestamp?

    public let ledgerId: LedgerId

    /// Returns the transaction associated with this schedule
    ///
    /// This function may or may not be O(1)
    ///
    /// This function may or may not throw
    ///
    /// This function name is not final.
    public func getScheduledTransaction() throws -> Transaction {
        self.scheduledTransaction.transaction
    }

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try .fromJsonBytes(bytes)
    }

    public func toBytes() -> Data {
        // can't have `throws` because that's the wrong function signature.
        // swiftlint:disable force_try
        try! toJsonBytes()
    }
}

extension ScheduleInfo: ToFromJsonBytes {
    internal static var cFromBytes: FromJsonBytesFunc { hedera_schedule_info_from_bytes }
    internal static var cToBytes: ToJsonBytesFunc { hedera_schedule_info_to_bytes }
}
