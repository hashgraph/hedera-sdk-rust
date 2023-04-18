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

import GRPC
import HederaProtobufs

public final class PrngTransaction: Transaction {
    public override init() {
        super.init()
    }

    internal init(protobuf proto: Proto_TransactionBody, _ data: Proto_UtilPrngTransactionBody) throws {
        self.range = data.range != 0 ? UInt32(bitPattern: data.range) : nil
        try super.init(protobuf: proto)
    }

    /// The upper-bound for the random number.
    ///
    /// If the value is zero or `nil`, instead of returning a 32-bit number, a 384-bit number will be returned.
    public var range: UInt32? {
        willSet {
            ensureNotFrozen(fieldName: "range")
        }
    }

    /// Sets the upper-bound for the random number.
    ///
    /// If the value is zero, instead of returning a 32-bit number, a 384-bit number will be returned.
    @discardableResult
    public func range(_ range: UInt32) -> Self {
        self.range = range

        return self
    }

    internal override func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        _ = chunkInfo.assertSingleTransaction()

        return .utilPrng(toProtobuf())
    }

    internal override func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        try await Proto_UtilServiceAsyncClient(channel: channel).prng(request)
    }
}

extension PrngTransaction: ToProtobuf {
    internal typealias Protobuf = Proto_UtilPrngTransactionBody

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            if let range = self.range {
                proto.range = Int32(bitPattern: range)
            }
        }
    }
}

extension PrngTransaction {
    internal func toSchedulableTransactionData() -> Proto_SchedulableTransactionBody.OneOf_Data {
        .utilPrng(toProtobuf())
    }
}
