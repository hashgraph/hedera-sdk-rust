import Foundation
import CHedera

/// The unique identifier for a smart contract on Hedera.
public final class ContractId: EntityId {
    public let evmAddress: Data?;

    public required init(shard: UInt64, realm: UInt64, num: UInt64) {
        evmAddress = nil
        super.init(shard: shard, realm: realm, num: num)
    }

    private convenience init(parsing description: String) throws {
        var hedera = HederaContractId();

        let err = hedera_contract_id_from_string(description, &hedera)

        if err != HEDERA_ERROR_OK {
            throw HError(err)!
        }

        self.init(unsafeFromCHedera: hedera)
    }

    public required convenience init?(_ description: String) {
        try? self.init(parsing: description)
    }

    internal init(unsafeFromCHedera hedera: HederaContractId) {
        if let evmAddress = hedera.evm_address {
            self.evmAddress = Data(bytesNoCopy: evmAddress, count: 20, deallocator: Data.unsafeCHederaBytesFree)

            super.init(shard: hedera.shard, realm: hedera.realm, num: 0)
        } else {
            evmAddress = nil
            super.init(shard: hedera.shard, realm: hedera.realm, num: hedera.num)
        }
    }

    internal func unsafeWithCHedera<Result>(_ body: (HederaContractId) throws -> Result) rethrows -> Result {
        assert(self.evmAddress.map { $0.count == 20 } ?? true);

        if let evmAddress = self.evmAddress {
            return try evmAddress.withUnsafeTypedBytes { evmAddress in
                return try body(HederaContractId(
                    shard: self.shard,
                    realm: self.realm,
                    num: self.num,
                    evm_address: UnsafeMutablePointer(mutating: evmAddress.baseAddress)
                ))
            }
        } else {
            return try body(HederaContractId(shard: self.shard, realm: self.realm, num: self.num, evm_address: nil))
        }
    }

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try bytes.withUnsafeTypedBytes { pointer in
            var hedera = HederaContractId();

            let err = hedera_contract_id_from_bytes(pointer.baseAddress, pointer.count, &hedera)

            if err != HEDERA_ERROR_OK {
                throw HError(err)!
            }

            return Self(unsafeFromCHedera: hedera)
        }
    }

    public static func fromString(_ description: String) throws -> Self {
        try Self(parsing: description)
    }

    public func toBytes() -> Data {
        unsafeWithCHedera { hedera in
            var buf: UnsafeMutablePointer<UInt8>?
            let size = hedera_contract_id_to_bytes(hedera, &buf)

            return Data(bytesNoCopy: buf!, count: size, deallocator: Data.unsafeCHederaBytesFree)
        }
    }

    public override var description: String {
        if let evmAddress = evmAddress {
            return "\(shard).\(realm).\(evmAddress)"
        } else {
            return super.description
        }
    }
}
