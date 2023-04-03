import Foundation
import HederaProtobufs

/// A transfer of ``Hbar`` that occured within a ``Transaction``
///
/// Returned as part of a ``TransactionRecord``
public struct Transfer {
    /// The account ID that this transfer is to/from.
    public let accountId: AccountId

    /// The value of this transfer.
    ///
    /// Negative if the account sends/withdraws hbar, positive if it receives hbar.
    public let amount: Hbar
}

extension Transfer: TryProtobufCodable {
    internal typealias Protobuf = Proto_AccountAmount

    internal init(protobuf proto: Protobuf) throws {
        self.init(
            accountId: try .fromProtobuf(proto.accountID),
            amount: .fromTinybars(proto.amount)
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.accountID = accountId.toProtobuf()
            proto.amount = amount.toTinybars()
        }
    }
}
