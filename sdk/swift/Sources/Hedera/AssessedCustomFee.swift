import CHedera
import Foundation
import HederaProtobufs

/// A custom transfer fee that was assessed during the handling of a ``TransferTransaction``.
public struct AssessedCustomFee: Equatable, Codable {
    /// The amount of currency charged to each payer.
    public let amount: Int64

    /// The currency `amount` is charged in, if `None` the fee is in HBar.
    public let tokenId: TokenId?

    /// The account that receives the fees that were charged.
    public let feeCollectorAccountId: AccountId?

    /// A list of all accounts that were charged this fee.
    public let payerAccountIdList: [AccountId]

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(fromProtobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }
}

extension AssessedCustomFee: TryProtobufCodable {
    internal typealias Protobuf = Proto_AssessedCustomFee

    internal init(fromProtobuf proto: Protobuf) throws {
        let tokenId = proto.hasTokenID ? proto.tokenID : nil
        let feeCollectorAccountId = proto.hasFeeCollectorAccountID ? proto.feeCollectorAccountID : nil

        self.init(
            amount: proto.amount,
            tokenId: .fromProtobuf(tokenId),
            feeCollectorAccountId: try .fromProtobuf(feeCollectorAccountId),
            payerAccountIdList: try .fromProtobuf(proto.effectivePayerAccountID)
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.amount = amount
            tokenId?.toProtobufInto(&proto.tokenID)
            feeCollectorAccountId?.toProtobufInto(&proto.feeCollectorAccountID)
            proto.effectivePayerAccountID = payerAccountIdList.toProtobuf()
        }
    }
}
