import HederaProtobufs

public struct HbarAllowance: ValidateChecksums {
    public let ownerAccountId: AccountId
    public let spenderAccountId: AccountId
    public let amount: Hbar

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try ownerAccountId.validateChecksums(on: ledgerId)
        try spenderAccountId.validateChecksums(on: ledgerId)
    }
}

extension HbarAllowance: TryProtobufCodable {
    internal typealias Protobuf = Proto_CryptoAllowance

    internal init(protobuf proto: Protobuf) throws {
        self.init(
            ownerAccountId: try .fromProtobuf(proto.owner),
            spenderAccountId: try .fromProtobuf(proto.spender),
            amount: .fromTinybars(proto.amount)
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.owner = ownerAccountId.toProtobuf()
            proto.spender = spenderAccountId.toProtobuf()
            proto.amount = amount.toTinybars()
        }
    }
}
