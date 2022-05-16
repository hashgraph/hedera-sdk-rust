import CHedera

/// A private key on the Hedera network.
public final class PublicKey: LosslessStringConvertible {
    private let ptr: OpaquePointer

    internal init(_ ptr: OpaquePointer) {
        self.ptr = ptr
    }

    public init?(_ description: String) {
        var key = OpaquePointer.init(bitPattern: 0)
        let err = hedera_public_key_from_string(description, &key)

        if err != HEDERA_ERROR_OK {
            return nil
        }

        ptr = key!
    }

    deinit {
        hedera_public_key_free(ptr)
    }

    public var description: String {
        let descriptionBytes = hedera_public_key_to_string(ptr)
        let description = String(validatingUTF8: descriptionBytes!)!

        return description
    }
}
