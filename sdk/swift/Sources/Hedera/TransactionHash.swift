import Foundation
import CryptoSwift

public struct TransactionHash {
    internal let data: Data

    internal static func generate(_ transactionBytes: Data) -> Self {
        Self(data: Data(CryptoSwift.SHA2(variant: .sha384).calculate(for: transactionBytes.bytes)))
    }
}
