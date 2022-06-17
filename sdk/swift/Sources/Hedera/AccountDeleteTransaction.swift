import Foundation

/// Mark an account as deleted, moving all its current hbars to another account.
///
/// It will remain in the ledger, marked as deleted, until it expires.
/// Transfers into it a deleted account will fail.
///
public class AccountDeleteTransaction: Transaction {
    /// Create a new `AccountDeleteTransaction` ready for configuration.
    public override init() {}

    /// The account ID which will receive all remaining hbars.
    public var transferAccountId: AccountAddress?

    /// Sets the account ID which will receive all remaining hbars.
    @discardableResult
    public func transferAccountId(_ transferAccountId: AccountAddress) -> Self {
        self.transferAccountId = transferAccountId

        return self
    }

    /// The account ID which should be deleted.
    public var deleteAccountId: AccountAddress?

    /// Sets the account ID which should be deleted.
    @discardableResult
    public func deleteAccountId(_ deleteAccountId: AccountAddress) -> Self {
        self.deleteAccountId = deleteAccountId

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case transferAccountId
        case deleteAccountId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(transferAccountId, forKey: .transferAccountId)
        try container.encodeIfPresent(deleteAccountId, forKey: .deleteAccountId)

        try super.encode(to: encoder)
    }
}
