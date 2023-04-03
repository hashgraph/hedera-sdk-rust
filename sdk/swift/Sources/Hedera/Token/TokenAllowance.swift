import HederaProtobufs

public struct TokenAllowance: ValidateChecksums {
    public let tokenId: TokenId
    public let ownerAccountId: AccountId
    public let spenderAccountId: AccountId
    public let amount: UInt64

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try tokenId.validateChecksums(on: ledgerId)
        try ownerAccountId.validateChecksums(on: ledgerId)
        try spenderAccountId.validateChecksums(on: ledgerId)
    }
}

extension TokenAllowance: TryProtobufCodable {
    internal typealias Protobuf = Proto_TokenAllowance

    internal init(protobuf proto: Protobuf) throws {
        self.init(
            tokenId: .fromProtobuf(proto.tokenID),
            ownerAccountId: try .fromProtobuf(proto.owner),
            spenderAccountId: try .fromProtobuf(proto.spender),
            amount: UInt64(proto.amount)
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.tokenID = tokenId.toProtobuf()
            proto.owner = ownerAccountId.toProtobuf()
            proto.spender = spenderAccountId.toProtobuf()
            proto.amount = Int64(amount)
        }
    }
}
