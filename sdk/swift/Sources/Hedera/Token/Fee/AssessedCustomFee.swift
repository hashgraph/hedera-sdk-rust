import CHedera
import Foundation

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
        try Self.fromJsonBytes(bytes)
    }

    public func toBytes() -> Data {
        // can't have `throws` because that's the wrong function signature.
        // swiftlint:disable force_try
        try! toJsonBytes()
    }
}

extension AssessedCustomFee: ToFromJsonBytes {
    internal static var cFromBytes: FromJsonBytesFunc { hedera_assessed_custom_fee_from_bytes }
    internal static var cToBytes: ToJsonBytesFunc { hedera_assessed_custom_fee_to_bytes }
}
