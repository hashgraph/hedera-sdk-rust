import Foundation

/// Delete a file or smart contract - can only be done with a Hedera admin.
public final class SystemDeleteTransaction: Transaction {
    /// Create a new `SystemDeleteTransaction`.
    public init(
        fileId: FileId? = nil,
        contractId: ContractId? = nil,
        expirationTime: Date? = nil
    ) {
        self.fileId = fileId
        self.contractId = contractId
        self.expirationTime = expirationTime
    }

    /// The file ID which should be deleted.
    public var fileId: FileId?

    /// Sets the file ID which should be deleted.
    @discardableResult
    public func fileId(_ fileId: FileId) -> Self {
        self.fileId = fileId

        return self
    }

    /// The contract ID which should be deleted.
    public var contractId: ContractId?

    /// Sets the contract ID which should be deleted.
    @discardableResult
    public func contractId(_ contractId: ContractId) -> Self {
        self.contractId = contractId

        return self
    }

    /// The timestamp at which the "deleted" file should
    /// truly be permanently deleted.
    public var expirationTime: Date?

    /// Sets the timestamp at which the "deleted" file should
    /// truly be permanently deleted.
    @discardableResult
    public func expirationTime(_ expirationTime: Date) -> Self {
        self.expirationTime = expirationTime

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case fileId
        case contractId
        case expirationTime
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(fileId, forKey: .fileId)
        try container.encodeIfPresent(contractId, forKey: .contractId)
        try container.encodeIfPresent(expirationTime?.unixTimestampNanos, forKey: .expirationTime)

        try super.encode(to: encoder)
    }
}
