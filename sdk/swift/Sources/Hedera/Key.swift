public enum Key {
    case single(PublicKey)
    case contractId(ContractId)
    case delegatableContractId(ContractId)
}

extension Key: Encodable {
    private enum KeyKeys: CodingKey {
        case single
        case contractId
        case delegatableContractId
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: KeyKeys.self)

        switch self {
        case .single(let publicKey):
            try container.encode(publicKey, forKey: .single)

        case .contractId(let contractId):
            try container.encode(contractId, forKey: .contractId)

        case .delegatableContractId(let contractId):
            try container.encode(contractId, forKey: .delegatableContractId)
        }
    }
}
