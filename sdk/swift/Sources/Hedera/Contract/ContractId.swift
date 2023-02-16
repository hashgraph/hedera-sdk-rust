import CHedera
import Foundation

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
        switch try PartialEntityId<S.SubSequence>(parsing: description) {
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

            if evmAddress.count != 20 {
                throw HError(
                    kind: .basicParse,
                    description: "expected `20` byte evm address, got `\(evmAddress.count)` bytes")
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

    internal init(unsafeFromCHedera hedera: HederaContractId) {
        shard = hedera.shard
        realm = hedera.realm

        if let evmAddress = hedera.evm_address {
            self.num = 0
            self.evmAddress = Data(bytesNoCopy: evmAddress, count: 20, deallocator: .unsafeCHederaBytesFree)
        } else {
            self.num = hedera.num
            self.evmAddress = nil
        }

        self.checksum = nil
    }

    internal func unsafeWithCHedera<Result>(_ body: (HederaContractId) throws -> Result) rethrows -> Result {
        assert(self.evmAddress.map { $0.count == 20 } ?? true)

        if let evmAddress = self.evmAddress {
            return try evmAddress.withUnsafeTypedBytes { evmAddress in
                return try body(
                    HederaContractId(
                        shard: self.shard,
                        realm: self.realm,
                        num: self.num,
                        evm_address: UnsafeMutablePointer(mutating: evmAddress.baseAddress)
                    ))
            }
        }

        return try body(HederaContractId(shard: self.shard, realm: self.realm, num: self.num, evm_address: nil))
    }

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try bytes.withUnsafeTypedBytes { pointer in
            var hedera = HederaContractId()

            try HError.throwing(error: hedera_contract_id_from_bytes(pointer.baseAddress, pointer.count, &hedera))

            return Self(unsafeFromCHedera: hedera)
        }
    }

    public static func fromEvmAddress(_ shard: UInt64, _ realm: UInt64, _ address: String) throws -> Self {
        Self(shard: shard, realm: realm, evmAddress: try SolidityAddress(parsing: address).data)
    }

    public func toSolidityAddress() throws -> String {
        if let evmAddress = evmAddress {
            return evmAddress.hexStringEncoded()
        }

        return String(describing: try SolidityAddress(self))

    }

    public func toBytes() -> Data {
        unsafeWithCHedera { hedera in
            var buf: UnsafeMutablePointer<UInt8>?
            let size = hedera_contract_id_to_bytes(hedera, &buf)

            return Data(bytesNoCopy: buf!, count: size, deallocator: .unsafeCHederaBytesFree)
        }
    }

    public var description: String {
        if let evmAddress = evmAddress {
            return "\(shard).\(realm).\(evmAddress)"
        } else {
            return helper.description
        }
    }

    public func toStringWithChecksum(_ client: Client) -> String {
        precondition(evmAddress == nil, "cannot create a checksum for a `ContractId` with an evmAddress")

        return helper.toStringWithChecksum(client)
    }

    public func validateChecksum(_ client: Client) throws {
        try validateChecksums(on: client.ledgerId!)
    }

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        if evmAddress != nil {
            return
        }

        try helper.validateChecksum(on: ledgerId)
    }
}
