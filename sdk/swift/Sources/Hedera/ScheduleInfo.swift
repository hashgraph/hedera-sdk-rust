import Foundation

/// Response from `ScheduleInfoQuery`.
public final class ScheduleInfo: Codable {
    /// The ID of the schedule for which information is requested.
    public let scheduleId: ScheduleId

    /// The account that created the scheduled transaction.
    public let creatorAccountId: AccountId

    /// The account paying for the execution of the scheduled transaction.
    public let payerAccountId: AccountId?

    /// The signatories that have provided signatures so far for the schedule
    /// transaction.
    public let signatories: [Key]

    /// The key which is able to delete the schedule transaction if set.
    public let adminKey: Key?

    /// The transaction id that will be used in the record of the scheduled transaction (if
    /// it executes).
    // TODO: TransactionId type
    public let scheduledTransactionId: String

    /// When set to true, the transaction will be evaluated for execution at `expiration_time`
    /// instead of when all required signatures are received.
    public let waitForExpiry: Bool

    /// Publicly visible information about the Schedule entity.
    public let scheduleMemo: String

    /// The date and time the schedule transaction will expire
    public let expirationTime: Date?

    /// The time the schedule transaction was executed.
    public let executedAt: Date?

    /// The time the schedule transaction was deleted.
    public let deletedAt: Date?
}
