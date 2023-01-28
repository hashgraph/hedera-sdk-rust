import Foundation

public struct TransactionHash {
    internal let data: Data

    internal static func generate(_ transactionBytes: Data) -> Self {
        Self(data: Crypto.Sha2.sha384(transactionBytes))
    }
}
