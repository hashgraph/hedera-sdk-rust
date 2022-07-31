import Foundation

/// Create a new schedule entity (or simply, schedule) in the network's action queue.
///
/// Upon `SUCCESS`, the receipt contains the `ScheduleId` of the created schedule. A schedule
/// entity includes a `scheduled_transaction_body` to be executed.
///
/// When the schedule has collected enough signing keys to satisfy the schedule's signing
/// requirements, the schedule can be executed.
///
public final class ScheduleCreateTransaction: Transaction {
    /// Create a new `ScheduleCreateTransaction`.
    public init(
        expirationTime: Date? = nil,
        isWaitForExpiry: Bool = false,
        payerAccountId: AccountId? = nil,
        scheduledTransaction: Transaction? = nil,
        adminKey: Key? = nil,
        scheduleMemo: String = ""
    ) {
        self.expirationTime = expirationTime
        self.isWaitForExpiry = isWaitForExpiry
        self.payerAccountId = payerAccountId
        self.scheduledTransaction = scheduledTransaction
        self.adminKey = adminKey
        self.scheduleMemo = scheduleMemo
    }

    /// The timestamp for when the transaction should be evaluated for execution and then expire.
    public var expirationTime: Date?

    /// Set the timestamp for when the transaction should be evaluated for execution and then expire.
    @discardableResult
    public func expirationTime(_ expirationTime: Date?) -> Self {
        self.expirationTime = expirationTime

        return self
    }

    /// If true, the transaction will be evaluated for execution at expiration_time instead
    /// of when all required signatures are received.
    public var isWaitForExpiry: Bool

    /// Set if the transaction will be evaluated for execution at expiration_time instead
    /// of when all required signatures are received.
    @discardableResult
    public func isWaitForExpiry(_ isWaitForExpiry: Bool) -> Self {
        self.isWaitForExpiry = isWaitForExpiry

        return self
    }

    /// The id of the account to be charged the service fee for the scheduled transaction at
    /// the consensus time that it executes (if ever).
    public var payerAccountId: AccountId?

    /// Set the id of the account to be charged the service fee for the scheduled transaction at
    /// the consensus time that it executes (if ever).
    @discardableResult
    public func payerAccountId(_ payerAccountId: AccountId?) -> Self {
        self.payerAccountId = payerAccountId

        return self
    }

    /// The scheduled transaction.
    public var scheduledTransaction: Transaction?

    /// Set the scheduled transaction.
    @discardableResult
    public func scheduledTransaction(_ scheduledTransaction: Transaction?) -> Self {
        self.scheduledTransaction = scheduledTransaction

        return self
    }

    /// The Hedera key which can be used to sign a ScheduleDelete and remove the schedule.
    public var adminKey: Key?

    /// Set the Hedera key which can be used to sign a ScheduleDelete and remove the schedule.
    @discardableResult
    public func adminKey(_ adminKey: Key?) -> Self {
        self.adminKey = adminKey

        return self
    }

    /// The memo for the schedule entity.
    public var scheduleMemo: String

    /// Set the memo for the schedule entity.
    @discardableResult
    public func scheduleMemo(_ scheduleMemo: String) -> Self {
        self.scheduleMemo = scheduleMemo

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case expirationTime
        case isWaitForExpiry
        case payerAccountId
        case scheduledTransaction
        case adminKey
        case scheduleMemo
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(expirationTime?.unixTimestampNanos, forKey: .expirationTime)
        try container.encode(isWaitForExpiry, forKey: .isWaitForExpiry)
        try container.encodeIfPresent(payerAccountId, forKey: .payerAccountId)
        try container.encodeIfPresent(scheduledTransaction, forKey: .scheduledTransaction)
        try container.encodeIfPresent(adminKey, forKey: .adminKey)
        try container.encode(scheduleMemo, forKey: .scheduleMemo)

        try super.encode(to: encoder)
    }
}
