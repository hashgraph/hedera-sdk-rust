import Foundation

internal struct SolidityAddress: CustomStringConvertible {
    internal let data: Data

    internal init(_ data: Data) throws {
        guard data.count == 20 else {
            throw HError(kind: .basicParse, description: "expected evm address to have 20 bytes, it had \(data.count)")
        }

        self.data = data
    }

    internal init<E: EntityId>(_ id: E) throws {
        guard let shard = UInt32(exactly: id.shard) else {
            // todo: use a proper error kind
            throw HError(kind: .basicParse, description: "Shard too big for `toSolidityAddress`")
        }

        self.data = (shard.bigEndianBytes + id.realm.bigEndianBytes + id.num.bigEndianBytes)
    }

    internal init<S: StringProtocol>(parsing description: S) throws {
        let description = description.stripPrefix("0x") ?? description[...]

        guard let bytes = Data(hexEncoded: description) else {
            // todo: better error message
            throw HError(kind: .basicParse, description: "invalid solidity address")
        }

        try self.init(bytes)
    }

    internal func toEntityId<E: EntityId>() -> E {
        let shard = UInt32(bigEndianBytes: data[..<4])!
        // eww copies, but, what can we do?
        let realm = UInt64(bigEndianBytes: Data(data[4..<12]))!
        let num = UInt64(bigEndianBytes: Data(data[12...]))!

        return E(shard: UInt64(shard), realm: realm, num: num)
    }

    internal var description: String {
        data.hexStringEncoded()
    }
}
