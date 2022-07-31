import CHedera

/// The unique identifier for a cryptocurrency account on Hedera.
public final class AccountId: EntityId {
    public let alias: PublicKey?

    public init(shard: UInt64 = 0, realm: UInt64 = 0, alias: PublicKey) {
        self.alias = alias

        super.init(shard: shard, realm: realm, num: 0)
    }

    public required init(shard: UInt64 = 0, realm: UInt64 = 0, num: UInt64) {
        alias = nil

        super.init(shard: shard, realm: realm, num: num)
    }

    public required init?(_ description: String) {
        var shard: UInt64 = 0
        var realm: UInt64 = 0
        var num: UInt64 = 0
        var alias: OpaquePointer?

        let err = hedera_account_id_from_string(description, &shard, &realm, &num, &alias)

        if err != HEDERA_ERROR_OK {
            return nil
        }

        self.alias = alias != nil ? PublicKey(alias!) : nil

        super.init(shard: shard, realm: realm, num: num)
    }

    public required convenience init(integerLiteral value: IntegerLiteralType) {
        self.init(num: UInt64(value))
    }

    public required convenience init(stringLiteral value: StringLiteralType) {
        self.init(value)!
    }

    public required convenience init(from decoder: Decoder) throws {
        self.init(try decoder.singleValueContainer().decode(String.self))!
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        try container.encode(String(describing: self))
    }

    public override var description: String {
        if let alias = alias {
            return "\(shard).\(realm).\(alias)"
        }

        return super.description
    }

}

// TODO: checksum
// TODO: to evm address
// TODO: hash
// TODO: equals
