import Foundation

/// A transfer of ``Hbar`` that occured within a ``Transaction``
///
/// Returned as part of a ``TransactionRecord``
public struct Transfer: Codable {
    /// The account ID that this transfer is to/from.
    public let accountId: AccountId

    /// The value of this transfer.
    ///
    /// Negative if the account sends/withdraws hbar, positive if it receives hbar.
    public let amount: Hbar
}
