import Foundation

internal class Signer {
    internal init(_ publicKey: PublicKey, _ signFunc: @escaping (Data) -> Data) {
        self.publicKey = publicKey
        self.signFunc = signFunc
    }

    internal let publicKey: PublicKey
    fileprivate let signFunc: (Data) -> Data

    func callAsFunction(_ message: Data) -> (PublicKey, Data) {
        (publicKey, signFunc(message))
    }
}
