import CHedera

/// A public key on the Hedera network.
public final class PublicKey: LosslessStringConvertible, ExpressibleByStringLiteral, Codable {
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

    public required convenience init(stringLiteral value: StringLiteralType) {
        self.init(value)!
    }

    public required convenience init(from decoder: Decoder) throws {
        self.init(try decoder.singleValueContainer().decode(String.self))!
    }

    deinit {
        hedera_public_key_free(ptr)
    }

    public var description: String {
        let descriptionBytes = hedera_public_key_to_string(ptr)
        let description = String(validatingUTF8: descriptionBytes!)!

        return description
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        try container.encode(String(describing: self))
    }
}
