import Foundation

/// Undelete a file or smart contract that was deleted by SystemDelete.
public final class SystemUndeleteTransaction: Transaction {
    /// Create a new `SystemUndeleteTransaction`.
    public init(
        fileId: FileId? = nil,
        contractId: ContractId? = nil
    ) {
        self.fileId = fileId
        self.contractId = contractId
    }

    /// The file ID to undelete.
    public var fileId: FileId?

    /// Sets the file ID to undelete.
    @discardableResult
    public func fileId(_ fileId: FileId) -> Self {
        self.fileId = fileId

        return self
    }

    /// The contract ID to undelete.
    public var contractId: ContractId?

    /// Sets the contract ID to undelete.
    @discardableResult
    public func contractId(_ contractId: ContractId) -> Self {
        self.contractId = contractId

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case fileId
        case contractId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(fileId, forKey: .fileId)
        try container.encodeIfPresent(contractId, forKey: .contractId)

        try super.encode(to: encoder)
    }
}
