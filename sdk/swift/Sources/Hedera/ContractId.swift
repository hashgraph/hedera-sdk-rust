import CHedera

/// Either ``ContractId`` or ``ContractEvmAddress``.
public class ContractIdOrEvmAddress: Encodable {
    /// The shard number (non-negative).
    public let shard: UInt64

    /// The realm number (non-negative).
    public let realm: UInt64

    fileprivate init(shard: UInt64, realm: UInt64) {
        self.shard = shard
        self.realm = realm
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        try container.encode(String(describing: self))
    }
}

/// The unique identifier for a smart contract on Hedera.
public final class ContractId: ContractIdOrEvmAddress, Decodable {
    public let num: UInt64

    public init(num: UInt64, shard: UInt64 = 0, realm: UInt64 = 0) {
        self.num = num
        super.init(shard: shard, realm: realm)
    }

    public init?(_ description: String) {
        var contractId = HederaContractId()
        let err = hedera_contract_id_from_string(description, &contractId)

        if err != HEDERA_ERROR_OK {
            return nil
        }

        num = contractId.num
        super.init(shard: contractId.shard, realm: contractId.realm)
    }

    public convenience init(from decoder: Decoder) throws {
        self.init(try decoder.singleValueContainer().decode(String.self))!
    }

    public var description: String {
        "\(shard).\(realm).\(num)"
    }
}

/// The identifier for a smart contract represented with an EVM address instead of a
/// contract number.
public class ContractEvmAddress: ContractIdOrEvmAddress {
    // TODO: EvmAddress
    public let address: [UInt8]

    public init(address: [UInt8], shard: UInt64 = 0, realm: UInt64 = 0) {
        self.address = address
        super.init(shard: shard, realm: realm)
    }
}

// TODO: checksum
// TODO: from string
// TODO: to string
// TODO: to evm address
// TODO: hash
// TODO: equals
