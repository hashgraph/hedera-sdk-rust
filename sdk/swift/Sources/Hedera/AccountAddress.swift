import CHedera

/// Either `AccountId` or `AccountAlias`. Some transactions and queries
/// accept `AccountAddress` as an input. All transactions and queries
/// return only `AccountId` as an output however.
public enum AccountAddress: LosslessStringConvertible, ExpressibleByIntegerLiteral, ExpressibleByStringLiteral, Codable
{
    case accountId(AccountId)
    case accountAlias(AccountAlias)

    public init(_ accountAlias: AccountAlias) {
        self = .accountAlias(accountAlias)
    }

    public init(_ accountId: AccountId) {
        self = .accountId(accountId)
    }

    public init?(_ description: String) {
        var shard: UInt64 = 0
        var realm: UInt64 = 0
        var num: UInt64 = 0
        var alias: OpaquePointer?

        let err = hedera_account_address_from_string(description, &shard, &realm, &num, &alias)

        if err != HEDERA_ERROR_OK {
            return nil
        }

        if alias == nil {
            self = .accountId(AccountId(shard: shard, realm: realm, num: num))
        } else {
            self = .accountAlias(AccountAlias(shard: shard, realm: realm, alias: PublicKey(alias!)))
        }
    }

    public init(integerLiteral value: IntegerLiteralType) {
        self = .accountId(AccountId(num: UInt64(value)))
    }

    public init(stringLiteral value: StringLiteralType) {
        self.init(value)!
    }

    public init(from decoder: Decoder) throws {
        self.init(try decoder.singleValueContainer().decode(String.self))!
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        try container.encode(String(describing: self))
    }

    public var description: String {
        switch self {
        case .accountId(let accountId):
            return accountId.description

        case .accountAlias(let accountAlias):
            return accountAlias.description
        }
    }
}

/// The unique identifier for a cryptocurrency account represented with an
/// alias instead of an account number.
public final class AccountAlias: LosslessStringConvertible, ExpressibleByStringLiteral, Codable {
    /// The shard number (non-negative).
    public let shard: UInt64

    /// The realm number (non-negative).
    public let realm: UInt64

    public let alias: PublicKey

    public init(shard: UInt64 = 0, realm: UInt64 = 0, alias: PublicKey) {
        self.shard = shard
        self.realm = realm
        self.alias = alias
    }

    public convenience init?(_ description: String) {
        var shard: UInt64 = 0
        var realm: UInt64 = 0
        var alias: OpaquePointer?

        let err = hedera_account_alias_from_string(description, &shard, &realm, &alias)

        if err != HEDERA_ERROR_OK {
            return nil
        }

        self.init(shard: shard, realm: realm, alias: PublicKey(alias!))
    }

    public convenience init(stringLiteral value: StringLiteralType) {
        self.init(value)!
    }

    public convenience init(from decoder: Decoder) throws {
        self.init(try decoder.singleValueContainer().decode(String.self))!
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        try container.encode(String(describing: self))
    }

    public var description: String {
        "\(shard).\(realm).\(alias)"
    }
}

// TODO: checksum
// TODO: to evm address
// TODO: hash
// TODO: equals
