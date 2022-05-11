/// Either `AccountId` or `AccountAlias`. Some transactions and queries
/// accept `AccountIdOrAlias` as an input. All transactions and queries 
/// return only `AccountId` as an output however.
public class AccountIdOrAlias {
    /// The shard number (non-negative).
    public let shard: UInt64

    /// The realm number (non-negative).
    public let realm: UInt64

    fileprivate init(shard: UInt64, realm: UInt64) {
        self.shard = shard;
        self.realm = realm;
    }
}

/// The unique identifier for a cryptocurrency account on Hedera.
public class AccountId: AccountIdOrAlias {
    public let num: UInt64

    public init(num: UInt64, shard: UInt64 = 0, realm: UInt64 = 0) {
        self.num = num
        super.init(shard: shard, realm: realm)
    }
}

/// The unique identifier for a cryptocurrency account represented with an
/// alias instead of an account number.
public class AccountAlias: AccountIdOrAlias {
    // TODO: PublicKey
    public let alias: Bool

    public init(alias: Bool, shard: UInt64 = 0, realm: UInt64 = 0) {
        self.alias = alias
        super.init(shard: shard, realm: realm)
    }
}

// TODO: checksum
// TODO: from string
// TODO: to string
// TODO: to evm address
// TODO: hash
// TODO: equals
