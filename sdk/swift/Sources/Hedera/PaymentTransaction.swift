import Foundation

internal final class PaymentTransaction: Codable {
    internal var nodeAccountIds: [AccountId]?
    internal var amount: UInt64?
    internal var maxAmount: UInt64?
    internal var maxTransactionFee: UInt64?
    internal var transactionMemo: String?
    internal var payerAccountId: AccountId?
    internal var transactionId: String?
    internal var transactionValidDuration: TimeInterval?
    // TODO: private var paymentSigners: [OpaquePointer] = [];

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(nodeAccountIds, forKey: .nodeAccountIds)
        try container.encodeIfPresent(amount, forKey: .amount)
        try container.encodeIfPresent(maxAmount, forKey: .maxAmount)
        try container.encodeIfPresent(maxTransactionFee, forKey: .maxTransactionFee)
        try container.encodeIfPresent(transactionMemo, forKey: .transactionMemo)
        try container.encodeIfPresent(payerAccountId, forKey: .payerAccountId)
        try container.encodeIfPresent(transactionId, forKey: .transactionId)
        try container.encodeIfPresent(transactionValidDuration, forKey: .transactionValidDuration)
    }
}
