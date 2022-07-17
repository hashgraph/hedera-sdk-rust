// TODO: KeyList
// TODO: ThresholdKey
public enum Key {
    case single(PublicKey)
    case contractId(ContractId)
    case delegatableContractId(ContractId)
}

extension Key: Codable {
    private enum CodingKeys: CodingKey {
        case single
        case contractId
        case delegatableContractId
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        switch self {
        case .single(let publicKey):
            try container.encode(publicKey, forKey: .single)

        case .contractId(let contractId):
            try container.encode(contractId, forKey: .contractId)

        case .delegatableContractId(let contractId):
            try container.encode(contractId, forKey: .delegatableContractId)
        }
    }

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        if let single = try container.decodeIfPresent(PublicKey.self, forKey: .single) {
            self = .single(single)
        } else if let contractId = try container.decodeIfPresent(ContractId.self, forKey: .contractId) {
            self = .contractId(contractId)
        } else if let contractId = try container.decodeIfPresent(ContractId.self, forKey: .delegatableContractId) {
            self = .delegatableContractId(contractId)
        } else {
            fatalError("(BUG) unexpected variant for Key")
        }
    }
}
