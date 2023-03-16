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

import HederaProtobufs

extension Proto_Response {
    internal func header() throws -> Proto_ResponseHeader {
        guard let header = self.response?.responseHeader() else {
            throw HError.fromProtobuf("unexpected missing `header` in `Response`")
        }

        return header
    }
}

extension Proto_Response.OneOf_Response {
    internal func responseHeader() -> Proto_ResponseHeader {
        switch self {
        case .getByKey(let response): return response.header
        case .getBySolidityID(let response): return response.header
        case .contractCallLocal(let response): return response.header
        case .contractGetBytecodeResponse(let response): return response.header
        case .contractGetInfo(let response): return response.header
        case .contractGetRecordsResponse(let response): return response.header
        case .cryptogetAccountBalance(let response): return response.header
        case .cryptoGetAccountRecords(let response): return response.header
        case .cryptoGetInfo(let response): return response.header
        case .cryptoGetLiveHash(let response): return response.header
        case .cryptoGetProxyStakers(let response): return response.header
        case .fileGetContents(let response): return response.header
        case .fileGetInfo(let response): return response.header
        case .transactionGetReceipt(let response): return response.header
        case .transactionGetRecord(let response): return response.header
        case .transactionGetFastRecord(let response): return response.header
        case .consensusGetTopicInfo(let response): return response.header
        case .networkGetVersionInfo(let response): return response.header
        case .tokenGetInfo(let response): return response.header
        case .scheduleGetInfo(let response): return response.header
        case .tokenGetAccountNftInfos(let response): return response.header
        case .tokenGetNftInfo(let response): return response.header
        case .tokenGetNftInfos(let response): return response.header
        case .networkGetExecutionTime(let response): return response.header
        case .accountDetails(let response): return response.header
        }
    }
}
