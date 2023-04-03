import Foundation
import HederaProtobufs

/// The unique identifier for a smart contract on Hedera.
public struct ContractId: EntityId {
    public let shard: UInt64
    public let realm: UInt64
    public let num: UInt64
    public let evmAddress: Data?
    public let checksum: Checksum?

    public init(shard: UInt64 = 0, realm: UInt64 = 0, num: UInt64, checksum: Checksum?) {
        self.shard = shard
        self.realm = realm
        self.num = num
        evmAddress = nil
        self.checksum = checksum
    }

    public init(shard: UInt64 = 0, realm: UInt64 = 0, num: UInt64) {
        self.init(shard: shard, realm: realm, num: num, checksum: nil)
    }

    private init(shard: UInt64, realm: UInt64, evmAddress: Data) {
        assert(evmAddress.count == 20)
        self.shard = shard
        self.realm = realm
        num = 0
        self.evmAddress = evmAddress
        self.checksum = nil
    }

    public init<S: StringProtocol>(parsing description: S) throws {
        switch try PartialEntityId(parsing: description) {
        case .short(let num):
            self.init(num: num)

        case .long(let shard, let realm, let last, let checksum):
            if let num = UInt64(last) {
                self.init(shard: shard, realm: realm, num: num, checksum: checksum)
                return
            }

            // might have `evmAddress`
            guard let evmAddress = Data(hexEncoded: last.stripPrefix("0x") ?? last) else {
                throw HError(
                    kind: .basicParse,
                    description:
                        "expected `<shard>.<realm>.<num>` or `<shard>.<realm>.<evmAddress>`, got, \(description)")
            }

            guard evmAddress.count == 20 else {
                throw HError.basicParse("expected `20` byte evm address, got `\(evmAddress.count)` bytes")
            }

            guard checksum == nil else {
                throw HError(
                    kind: .basicParse, description: "checksum not supported with `<shard>.<realm>.<evmAddress>`")
            }

            self.init(shard: shard, realm: realm, evmAddress: evmAddress)

        case .other(let description):
            throw HError(
                kind: .basicParse,
                description: "expected `<shard>.<realm>.<num>` or `<shard>.<realm>.<evmAddress>`, got, \(description)")
        }
    }

    public static func fromEvmAddress(_ shard: UInt64, _ realm: UInt64, _ address: String) throws -> Self {
        Self(shard: shard, realm: realm, evmAddress: try SolidityAddress(parsing: address).data)
    }

    internal static func fromEvmAddressBytes(_ shard: UInt64, _ realm: UInt64, _ address: Data) throws -> Self {
        Self(shard: shard, realm: realm, evmAddress: try SolidityAddress(address).data)
    }

    public func toSolidityAddress() throws -> String {
        if let evmAddress = evmAddress {
            return evmAddress.hexStringEncoded()
        }

        return String(describing: try SolidityAddress(self))

    }

    public var description: String {
        guard let evmAddress = evmAddress else {
            return helper.description
        }
        return "\(shard).\(realm).\(evmAddress)"
    }

    public func toStringWithChecksum(_ client: Client) throws -> String {
        guard evmAddress == nil else {
            throw HError.cannotCreateChecksum
        }

        return helper.toStringWithChecksum(client)
    }

    public func validateChecksum(_ client: Client) throws {
        try validateChecksums(on: client.ledgerId!)
    }

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        guard evmAddress == nil else {
            return
        }

        try helper.validateChecksum(on: ledgerId)
    }

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(protobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }
}

extension ContractId: TryProtobufCodable {
    internal typealias Protobuf = Proto_ContractID

    internal init(protobuf proto: Protobuf) throws {
        let shard = UInt64(proto.shardNum)
        let realm = UInt64(proto.realmNum)
        guard let contract = proto.contract else {
            throw HError.fromProtobuf("unexpected missing `contract` field")
        }

        switch contract {
        case .contractNum(let num):
            self.init(shard: shard, realm: realm, num: UInt64(num))
        case .evmAddress(let evmAddress):
            self.init(shard: shard, realm: realm, evmAddress: evmAddress)
        }
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.shardNum = Int64(shard)
            proto.realmNum = Int64(realm)
            if let evmAddress = evmAddress {
                proto.evmAddress = evmAddress
            } else {
                proto.contractNum = Int64(num)
            }
        }
    }
}
