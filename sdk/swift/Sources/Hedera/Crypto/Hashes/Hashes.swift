import CHedera
import Foundation

extension Crypto {
    internal enum Sha3 {
        case keccak256
    }
}

extension Crypto.Sha3 {
    internal static func digest(_ kind: Self, _ data: Data) -> Data {
        kind.digest(data)
    }

    internal func digest(_ data: Data) -> Data {
        switch self {
        case .keccak256:
            return data.withUnsafeTypedBytes { buffer in
                var output: UnsafeMutablePointer<UInt8>?
                let count = hedera_crypto_sha3_keccak256_digest(buffer.baseAddress, buffer.count, &output)
                return Data(bytesNoCopy: output!, count: count, deallocator: .unsafeCHederaBytesFree)
            }
        }
    }

    /// Hash data using the `keccak256` algorithm.
    ///
    /// - Parameter data: the data to be hashed.
    ///
    /// - Returns: the hash of `data`.
    internal static func keccak256(_ data: Data) -> Data {
        digest(.keccak256, data)
    }
}
