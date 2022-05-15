import CHedera

/// A private key on the Hedera network.
public class PublicKey {
    private let ptr: OpaquePointer

    private init(_ ptr: OpaquePointer) {
        self.ptr = ptr
    }

    public init(_ description: String) {
        var key = OpaquePointer.init(bitPattern: 0)
        var _ = hedera_public_key_from_string(description, &key)

        // TODO: handle errors

        ptr = key!
    }

    deinit {
        hedera_public_key_free(ptr)
    }
}
