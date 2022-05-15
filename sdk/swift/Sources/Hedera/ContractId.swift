import CHedera

/// Either ``ContractId`` or ``ContractEvmAddress``.
public class ContractIdOrEvmAddress {
    /// The shard number (non-negative).
    public let shard: UInt64

    /// The realm number (non-negative).
    public let realm: UInt64

    fileprivate init(shard: UInt64, realm: UInt64) {
        self.shard = shard
        self.realm = realm
    }
}

/// The unique identifier for a smart contract on Hedera.
public final class ContractId: ContractIdOrEvmAddress {
    public let num: UInt64

    public init(num: UInt64, shard: UInt64 = 0, realm: UInt64 = 0) {
        self.num = num
        super.init(shard: shard, realm: realm)
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
