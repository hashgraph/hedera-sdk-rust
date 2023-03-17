/*
 * ‌
 * Hedera Swift SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

import CHedera
import Foundation

private typealias UnsafeFromBytesFunc = @convention(c) (
    UnsafePointer<UInt8>?, Int, UnsafeMutablePointer<OpaquePointer?>?
) -> HederaError

/// A private key on the Hedera network.
public final class PrivateKey: LosslessStringConvertible, ExpressibleByStringLiteral {
    internal let ptr: OpaquePointer

    private static func decodeBytes<S: StringProtocol>(_ description: S) throws -> Data {
        let description = description.stripPrefix("0x") ?? description[...]
        guard let bytes = Data(hexEncoded: description) else {
            throw HError(kind: .keyParse, description: "Invalid hex string")
        }

        return bytes
    }

    // sadly, we can't avoid a leaky abstraction here.
    internal static func unsafeFromPtr(_ ptr: OpaquePointer) -> Self {
        Self(ptr)
    }

    private init(_ ptr: OpaquePointer) {
        self.ptr = ptr
    }

    private init(bytes: Data, unsafeCallback chederaCallback: UnsafeFromBytesFunc) throws {
        self.ptr = try bytes.withUnsafeTypedBytes { pointer -> OpaquePointer in
            var key: OpaquePointer?
            try HError.throwing(error: chederaCallback(pointer.baseAddress, pointer.count, &key))

            return key!
        }
    }

    private convenience init(bytes: Data) throws {
        try self.init(bytes: bytes, unsafeCallback: hedera_private_key_from_bytes)
    }

    /// Generates a new Ed25519 private key.
    public static func generateEd25519() -> Self {
        self.init(hedera_private_key_generate_ed25519())
    }

    /// Generates a new ECDSA(secp256k1) private key.
    public static func generateEcdsa() -> Self {
        self.init(hedera_private_key_generate_ecdsa())
    }

    /// The ``PublicKey`` which corresponds to this private key.
    public var publicKey: PublicKey {
        PublicKey.unsafeFromPtr(hedera_private_key_get_public_key(ptr))
    }

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(bytes: bytes)
    }

    public static func fromBytesEd25519(_ bytes: Data) throws -> Self {
        try Self(bytes: bytes, unsafeCallback: hedera_private_key_from_bytes_ed25519)
    }

    public static func fromBytesEcdsa(_ bytes: Data) throws -> Self {
        try Self(bytes: bytes, unsafeCallback: hedera_private_key_from_bytes_ecdsa)
    }

    public static func fromBytesDer(_ bytes: Data) throws -> Self {
        try Self(bytes: bytes, unsafeCallback: hedera_private_key_from_bytes_der)
    }

    private convenience init<S: StringProtocol>(parsing description: S) throws {
        try self.init(bytes: Self.decodeBytes(description))
    }

    public static func fromString<S: StringProtocol>(_ description: S) throws -> Self {
        try Self(parsing: description)
    }

    public convenience init?(_ description: String) {
        try? self.init(parsing: description)
    }

    public required convenience init(stringLiteral value: StringLiteralType) {
        // swiftlint:disable:next force_try
        try! self.init(parsing: value)
    }

    public static func fromStringDer<S: StringProtocol>(_ description: S) throws -> Self {
        try fromBytesDer(decodeBytes(description))
    }

    public static func fromStringEd25519(_ description: String) throws -> Self {
        try fromBytesEd25519(decodeBytes(description))
    }

    public static func fromStringEcdsa(_ description: String) throws -> Self {
        try fromBytesEcdsa(decodeBytes(description))
    }

    /// Parse a `PrivateKey` from a [PEM](https://www.rfc-editor.org/rfc/rfc7468#section-10) encoded string.
    public static func fromPem(_ pem: String) throws -> Self {
        var key: OpaquePointer?
        try HError.throwing(error: hedera_private_key_from_pem(pem, &key))

        return Self(key!)
    }

    /// Parse a `PrivateKey` from a password protected [PEM](https://www.rfc-editor.org/rfc/rfc7468#section-11) encoded string.
    public static func fromPem(_ pem: String, _ password: String) throws -> Self {
        var key: OpaquePointer?
        try HError.throwing(error: hedera_private_key_from_pem_with_password(pem, password, &key))

        return Self(key!)
    }

    public func toBytesDer() -> Data {
        var buf: UnsafeMutablePointer<UInt8>?
        let size = hedera_private_key_to_bytes_der(ptr, &buf)

        return Data(bytesNoCopy: buf!, count: size, deallocator: .unsafeCHederaBytesFree)
    }

    public func toBytes() -> Data {
        var buf: UnsafeMutablePointer<UInt8>?
        let size = hedera_private_key_to_bytes(ptr, &buf)

        return Data(bytesNoCopy: buf!, count: size, deallocator: .unsafeCHederaBytesFree)
    }

    public func toBytesRaw() -> Data {
        var buf: UnsafeMutablePointer<UInt8>?
        let size = hedera_private_key_to_bytes_raw(ptr, &buf)

        return Data(bytesNoCopy: buf!, count: size, deallocator: .unsafeCHederaBytesFree)
    }

    public var description: String {
        toStringDer()
    }

    public func toString() -> String {
        String(describing: self)
    }

    public func toStringDer() -> String {
        toBytesDer().hexStringEncoded()
    }

    public func toStringRaw() -> String {
        toBytesRaw().hexStringEncoded()
    }

    public func toAccountId(shard: UInt64, realm: UInt64) -> AccountId {
        publicKey.toAccountId(shard: shard, realm: realm)
    }

    public func isEd25519() -> Bool {
        hedera_private_key_is_ed25519(ptr)
    }

    public func isEcdsa() -> Bool {
        hedera_private_key_is_ecdsa(ptr)
    }

    public func sign(_ message: Data) -> Data {
        message.withUnsafeTypedBytes { pointer in
            var buf: UnsafeMutablePointer<UInt8>?
            let size = hedera_private_key_sign(ptr, pointer.baseAddress, pointer.count, &buf)
            return Data(bytesNoCopy: buf!, count: size, deallocator: .unsafeCHederaBytesFree)
        }
    }

    public func isDerivable() -> Bool {
        hedera_private_key_is_derivable(ptr)
    }

    public func derive(_ index: Int32) throws -> Self {
        var derived: OpaquePointer?
        try HError.throwing(error: hedera_private_key_derive(ptr, index, &derived))

        return Self(derived!)
    }

    public func legacyDerive(_ index: Int64) throws -> Self {
        var derived: OpaquePointer?
        try HError.throwing(error: hedera_private_key_legacy_derive(ptr, index, &derived))

        return Self(derived!)
    }

    public static func fromMnemonic(_ mnemonic: Mnemonic, _ passphrase: String) -> Self {
        let seed = mnemonic.toSeed(passphrase: passphrase)
        return seed.withUnsafeTypedBytes { buffer in
            Self(hedera_private_key_from_mnemonic_seed(buffer.baseAddress, buffer.count))
        }
    }

    public static func fromMnemonic(_ mnemonic: Mnemonic) -> Self {
        Self.fromMnemonic(mnemonic, "")
    }

    public func signTransaction(_ transaction: Transaction) throws {
        try transaction.freeze()

        transaction.addSignatureSigner(.privateKey(self))
    }

    deinit {
        hedera_private_key_free(ptr)
    }
}
