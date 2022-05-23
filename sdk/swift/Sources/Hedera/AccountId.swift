import CHedera

/// Either `AccountId` or `AccountAlias`. Some transactions and queries
/// accept `AccountIdOrAlias` as an input. All transactions and queries
/// return only `AccountId` as an output however.
public class AccountIdOrAlias: Encodable {
    /// The shard number (non-negative).
    public let shard: UInt64

    /// The realm number (non-negative).
    public let realm: UInt64

    fileprivate init(shard: UInt64, realm: UInt64) {
        self.shard = shard
        self.realm = realm
    }
}

/// The unique identifier for a cryptocurrency account on Hedera.
public final class AccountId: AccountIdOrAlias, LosslessStringConvertible, Decodable {
    public let num: UInt64

    public init(num: UInt64, shard: UInt64 = 0, realm: UInt64 = 0) {
        self.num = num
        super.init(shard: shard, realm: realm)
    }

    public init?(_ description: String) {
        var accountId = HederaAccountId()
        let err = hedera_account_id_from_string(description, &accountId)

        if err != HEDERA_ERROR_OK {
            return nil
        }

        num = accountId.num
        super.init(shard: accountId.shard, realm: accountId.realm)
    }

    public convenience init(from decoder: Decoder) throws {
        self.init(try decoder.singleValueContainer().decode(String.self))!
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        try container.encode(String(describing: self))
    }

    public var description: String {
        "\(shard).\(realm).\(num)"
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

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        try container.encode(String(describing: self))
    }

    public var description: String {
        "\(shard).\(realm).\(alias)"
    }
}

// TODO: checksum
// TODO: from string
// TODO: to string
// TODO: to evm address
// TODO: hash
// TODO: equals
