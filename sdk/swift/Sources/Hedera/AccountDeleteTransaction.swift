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
    public private(set) var transferAccountId: AccountIdOrAlias?

    /// Sets the account ID which will receive all remaining hbars.
    @discardableResult
    public func transferAccountId(_ transferAccountId: AccountIdOrAlias) -> Self {
        self.transferAccountId = transferAccountId

        return self
    }

    /// The account ID which should be deleted.
    public private(set) var deleteAccountId: AccountIdOrAlias?

    /// Sets the account ID which should be deleted.
    @discardableResult
    public func deleteAccountId(_ deleteAccountId: AccountIdOrAlias) -> Self {
        self.deleteAccountId = deleteAccountId

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case transferAccountId
        case deleteAccountId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: AnyTransactionCodingKeys.self)
        var data = container.nestedContainer(keyedBy: CodingKeys.self, forKey: .accountDelete)

        try data.encodeIfPresent(transferAccountId, forKey: .transferAccountId)
        try data.encodeIfPresent(deleteAccountId, forKey: .deleteAccountId)

        try super.encode(to: encoder)
    }
}
