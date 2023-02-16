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

// todo: deduplicate these with `PrivateKey.swift`

private typealias UnsafeFromBytesFunc = @convention(c) (
    UnsafePointer<UInt8>?, Int, UnsafeMutablePointer<OpaquePointer?>?
) -> HederaError

/// A public key on the Hedera network.
public final class PublicKey: LosslessStringConvertible, ExpressibleByStringLiteral, Codable, Equatable, Hashable {
    internal let ptr: OpaquePointer

    // sadly, we can't avoid a leaky abstraction here.
    internal static func unsafeFromPtr(_ ptr: OpaquePointer) -> Self {
        Self(ptr)
    }

    private init(_ ptr: OpaquePointer) {
        self.ptr = ptr
    }

    private static func unsafeFromAnyBytes(_ bytes: Data, _ chederaCallback: UnsafeFromBytesFunc) throws -> Self {
        try bytes.withUnsafeTypedBytes { pointer -> Self in
            var key: OpaquePointer?

            try HError.throwing(error: chederaCallback(pointer.baseAddress, pointer.count, &key))

            return Self(key!)
        }
    }

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try unsafeFromAnyBytes(bytes, hedera_public_key_from_bytes)
    }

    public static func fromBytesEd25519(_ bytes: Data) throws -> Self {
        try unsafeFromAnyBytes(bytes, hedera_public_key_from_bytes_ed25519)
    }

    public static func fromBytesEcdsa(_ bytes: Data) throws -> Self {
        try unsafeFromAnyBytes(bytes, hedera_public_key_from_bytes_ecdsa)
    }

    public static func fromBytesDer(_ bytes: Data) throws -> Self {
        try unsafeFromAnyBytes(bytes, hedera_public_key_from_bytes_der)
    }

    private init(parsing description: String) throws {
        var key: OpaquePointer?
        try HError.throwing(error: hedera_public_key_from_string(description, &key))

        self.ptr = key!
    }

    public static func fromString(_ description: String) throws -> Self {
        try Self(parsing: description)
    }

    public convenience init?(_ description: String) {
        try? self.init(parsing: description)
    }

    public required convenience init(stringLiteral value: StringLiteralType) {
        // swiftlint:disable:next force_try
        try! self.init(parsing: value)
    }

    public static func fromStringDer(_ description: String) throws -> Self {
        var key: OpaquePointer?

        try HError.throwing(error: hedera_public_key_from_string_der(description, &key))

        return Self(key!)
    }

    public static func fromStringEd25519(_ description: String) throws -> Self {
        var key: OpaquePointer?

        try HError.throwing(error: hedera_public_key_from_string_ed25519(description, &key))

        return Self(key!)
    }

    public static func fromStringEcdsa(_ description: String) throws -> Self {
        var key: OpaquePointer?

        try HError.throwing(error: hedera_public_key_from_string_ecdsa(description, &key))
        return Self(key!)
    }

    public required convenience init(from decoder: Decoder) throws {
        self.init(try decoder.singleValueContainer().decode(String.self))!
    }

    public func toBytesDer() -> Data {
        var buf: UnsafeMutablePointer<UInt8>?
        let size = hedera_public_key_to_bytes_der(ptr, &buf)

        return Data(bytesNoCopy: buf!, count: size, deallocator: .unsafeCHederaBytesFree)
    }

    public func toBytes() -> Data {
        var buf: UnsafeMutablePointer<UInt8>?
        let size = hedera_public_key_to_bytes(ptr, &buf)

        return Data(bytesNoCopy: buf!, count: size, deallocator: .unsafeCHederaBytesFree)
    }

    public func toBytesRaw() -> Data {
        var buf: UnsafeMutablePointer<UInt8>?
        let size = hedera_public_key_to_bytes_raw(ptr, &buf)

        return Data(bytesNoCopy: buf!, count: size, deallocator: .unsafeCHederaBytesFree)
    }

    public var description: String {
        let descriptionBytes = hedera_public_key_to_string(ptr)
        return String(hString: descriptionBytes!)
    }

    public func toString() -> String {
        description
    }

    public func toStringDer() -> String {
        let stringBytes = hedera_public_key_to_string_der(ptr)
        return String(hString: stringBytes!)
    }

    public func toStringRaw() -> String {
        let stringBytes = hedera_public_key_to_string_raw(ptr)
        return String(hString: stringBytes!)
    }

    public func toAccountId(shard: UInt64, realm: UInt64) -> AccountId {
        AccountId(shard: shard, realm: realm, alias: self)
    }

    public func verify(_ message: Data, _ signature: Data) throws {
        try message.withUnsafeTypedBytes { messagePointer in
            try signature.withUnsafeTypedBytes { signaturePointer in
                let err = hedera_public_key_verify(
                    ptr, messagePointer.baseAddress, messagePointer.count, signaturePointer.baseAddress,
                    signaturePointer.count)

                try HError.throwing(error: err)
            }
        }
    }

    public func isEd25519() -> Bool {
        hedera_public_key_is_ed25519(ptr)
    }

    public func isEcdsa() -> Bool {
        hedera_public_key_is_ecdsa(ptr)
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        try container.encode(String(describing: self))
    }

    public static func == (lhs: PublicKey, rhs: PublicKey) -> Bool {
        // this will always be true for public keys, DER is a stable format with canonicalization.
        // ideally we'd do this a different way, but that needs to wait until ffi is gone.
        lhs.toBytesDer() == rhs.toBytesDer()
    }

    public func hash(into hasher: inout Hasher) {
        hasher.combine(toBytesDer())
    }

    /// Convert this public key into an evm address. The EVM address is This is the rightmost 20 bytes of the 32 byte Keccak-256 hash of the ECDSA public key.
    public func toEvmAddress() throws -> String {
        var stringPtr: UnsafeMutablePointer<CChar>?
        try HError.throwing(error: hedera_public_key_to_evm_address(ptr, &stringPtr))

        return String(hString: stringPtr!)
    }

    public func verifyTransaction(_ transaction: Transaction) throws {
        // we're a signer.
        if transaction.signers.contains(where: { self == $0.publicKey }) {
            return
        }

        guard let sources = transaction.sources else {
            throw HError(kind: .signatureVerify, description: "signer not in transaction")
        }

        try HError.throwing(error: hedera_public_key_verify_sources(ptr, sources.ptr))
    }

    deinit {
        hedera_public_key_free(ptr)
    }
}
