import HederaProtobufs
import SwiftProtobuf

public struct TokenNftAllowance: ValidateChecksums {
    public let tokenId: TokenId
    public let ownerAccountId: AccountId
    public let spenderAccountId: AccountId
    public var serials: [UInt64]
    public let approvedForAll: Bool?
    public let delegatingSpenderAccountId: AccountId?

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try tokenId.validateChecksums(on: ledgerId)
        try ownerAccountId.validateChecksums(on: ledgerId)
        try spenderAccountId.validateChecksums(on: ledgerId)
        try delegatingSpenderAccountId?.validateChecksums(on: ledgerId)
    }
}

extension TokenNftAllowance: TryProtobufCodable {
    internal typealias Protobuf = Proto_NftAllowance

    internal init(protobuf proto: Protobuf) throws {
        self.init(
            tokenId: .fromProtobuf(proto.tokenID),
            ownerAccountId: try .fromProtobuf(proto.owner),
            spenderAccountId: try .fromProtobuf(proto.spender),
            serials: proto.serialNumbers.map(UInt64.init),
            approvedForAll: proto.hasApprovedForAll ? proto.approvedForAll.value : nil,
            delegatingSpenderAccountId: proto.hasDelegatingSpender ? try .fromProtobuf(proto.delegatingSpender) : nil
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.tokenID = tokenId.toProtobuf()
            proto.owner = ownerAccountId.toProtobuf()
            proto.spender = spenderAccountId.toProtobuf()
            proto.serialNumbers = serials.map(Int64.init)

            if let approvedForAll = approvedForAll {
                proto.approvedForAll = Google_Protobuf_BoolValue(approvedForAll)
            }

            delegatingSpenderAccountId?.toProtobufInto(&proto.delegatingSpender)
        }
    }
}

public struct NftRemoveAllowance: ValidateChecksums {
    public let tokenId: TokenId
    public let ownerAccountId: AccountId
    public var serials: [UInt64]

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try tokenId.validateChecksums(on: ledgerId)
        try ownerAccountId.validateChecksums(on: ledgerId)
    }
}

extension NftRemoveAllowance: TryProtobufCodable {
    internal typealias Protobuf = Proto_NftRemoveAllowance

    internal init(protobuf proto: Protobuf) throws {
        self.init(
            tokenId: .fromProtobuf(proto.tokenID),
            ownerAccountId: try .fromProtobuf(proto.owner),
            serials: proto.serialNumbers.map(UInt64.init)
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.tokenID = tokenId.toProtobuf()
            proto.owner = ownerAccountId.toProtobuf()
            proto.serialNumbers = serials.map(Int64.init)
        }
    }
}
