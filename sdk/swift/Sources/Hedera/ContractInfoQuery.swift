/// Get information about a smart contract instance.
public final class ContractInfoQuery: Query<ContractInfo> {
    /// Create a new `ContractInfoQuery`.
    public init(
        contractId: ContractId? = nil
    ) {
        self.contractId = contractId
    }

    /// The contract ID for which information is requested.
    public var contractId: ContractId?

    /// Sets the contract ID for which information is requested.
    @discardableResult
    public func contractId(_ contractId: ContractId) -> Self {
        self.contractId = contractId

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case contractId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(contractId, forKey: .contractId)

        try super.encode(to: encoder)
    }
}
