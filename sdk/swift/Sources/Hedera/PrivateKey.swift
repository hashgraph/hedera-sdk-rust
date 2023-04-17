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

import CommonCrypto
import CryptoKit
import Foundation
import SwiftASN1
import secp256k1

internal struct Keccak256Digest: Crypto.SecpDigest {
    internal init?(_ bytes: Data) {
        guard bytes.count == Self.byteCount else {
            return nil
        }

        self.inner = bytes
    }

    fileprivate let inner: Data
    internal static let byteCount: Int = 32

    func withUnsafeBytes<R>(_ body: (UnsafeRawBufferPointer) throws -> R) rethrows -> R {
        try inner.withUnsafeBytes(body)
    }
}

private struct ChainCode {
    let data: Data
}

#if compiler(>=5.7)
    extension ChainCode: Sendable {}
#else
    extension ChainCode: @unchecked Sendable {}
#endif

/// A private key on the Hedera network.
public struct PrivateKey: LosslessStringConvertible, ExpressibleByStringLiteral, CustomStringConvertible,
    CustomDebugStringConvertible
{
    /// Debug description for `PrivateKey`
    ///
    /// Please note that debugDescriptions of any kind should not be considered a stable format.
    public var debugDescription: String {
        "PrivateKey(kind: \(String(reflecting: guts)), chainCode: \(String(describing: chainCode?.data))"
    }

    // we need to be sendable, so...
    // The idea being that we initialize the key whenever we need it, which is absolutely not free, but it is `Sendable`.
    fileprivate enum Repr: CustomDebugStringConvertible {
        fileprivate var debugDescription: String {
            switch self {
            case .ed25519:
                return "ed25519([redacted])"
            case .ecdsa:
                return "ecdsa([redacted])"
            }
        }

        case ed25519(Data)
        case ecdsa(Data)

        fileprivate init(kind: PrivateKey.Kind) {
            switch kind {
            case .ecdsa(let key): self = .ecdsa(key.rawRepresentation)
            case .ed25519(let key): self = .ed25519(key.rawRepresentation)
            }
        }

        fileprivate var kind: PrivateKey.Kind {
            // swiftlint:disable force_try
            switch self {
            case .ecdsa(let key): return .ecdsa(try! .init(rawRepresentation: key))
            case .ed25519(let key): return .ed25519(try! .init(rawRepresentation: key))
            }

            // swiftlint:enable force_try
        }
    }

    fileprivate enum Kind {
        case ed25519(CryptoKit.Curve25519.Signing.PrivateKey)
        case ecdsa(secp256k1.Signing.PrivateKey)
    }

    private init(kind: Kind, chainCode: Data? = nil) {
        self.guts = .init(kind: kind)
        self.chainCode = chainCode.map(ChainCode.init(data:))
    }

    private let guts: Repr

    private var kind: Kind {
        guts.kind
    }

    private let chainCode: ChainCode?

    private static func decodeBytes<S: StringProtocol>(_ description: S) throws -> Data {
        let description = description.stripPrefix("0x") ?? description[...]
        guard let bytes = Data(hexEncoded: description) else {
            throw HError(kind: .keyParse, description: "Invalid hex string")
        }

        return bytes
    }

    private init(bytes: Data) throws {
        try self.init(ed25519Bytes: bytes)
    }

    private init(ed25519Bytes bytes: Data) throws {
        guard bytes.count == 32 || bytes.count == 64 else {
            try self.init(derBytes: bytes)
            return
        }

        self.init(kind: .ed25519(try! .init(rawRepresentation: bytes.safeSubdata(in: 0..<32)!)))
    }

    private init(ecdsaBytes bytes: Data) throws {
        guard bytes.count == 32 else {
            try self.init(derBytes: bytes)
            return
        }

        do {
            self.init(kind: .ecdsa(try .init(rawRepresentation: bytes.safeSubdata(in: 0..<32)!)))
            return
        } catch {
            throw HError.keyParse(String(describing: error))
        }
    }

    private init(derBytes bytes: Data) throws {
        let info: Pkcs8.PrivateKeyInfo
        let inner: ASN1OctetString
        do {
            info = try .init(derEncoded: Array(bytes))
            // PrivateKey is an `OctetString`, and the `PrivateKey`s we all support are `OctetStrings`.
            // So, we, awkwardly, have an `OctetString` containing an `OctetString` containing our key material.
            inner = try .init(derEncoded: info.privateKey.bytes)
        } catch {
            throw HError.keyParse(String(describing: error))
        }

        switch info.algorithm.oid {
        case .NamedCurves.ed25519: try self.init(ed25519Bytes: Data(inner.bytes))
        case .NamedCurves.secp256k1: try self.init(ecdsaBytes: Data(inner.bytes))
        case let oid:
            throw HError.keyParse("unsupported key algorithm: \(oid)")
        }
    }

    /// Generates a new Ed25519 private key.
    public static func generateEd25519() -> Self {
        Self(kind: .ed25519(.init()), chainCode: .randomData(withLength: 32))
    }

    /// Generates a new ECDSA(secp256k1) private key.
    public static func generateEcdsa() -> Self {
        .ecdsa(try! .init())
    }

    internal static func ed25519(_ key: Curve25519.Signing.PrivateKey) -> Self {
        Self(kind: .ed25519(key))
    }

    internal static func ecdsa(_ key: secp256k1.Signing.PrivateKey) -> Self {
        Self(kind: .ecdsa(key))
    }

    /// The ``PublicKey`` which corresponds to this private key.
    public var publicKey: PublicKey {
        switch kind {
        case .ed25519(let key): return .ed25519(key.publicKey)
        case .ecdsa(let key): return .ecdsa(key.publicKey)

        }
    }

    private var algorithm: Pkcs5.AlgorithmIdentifier {
        let oid: ASN1ObjectIdentifier

        // todo: `self.kind`
        switch self.kind {
        case .ed25519: oid = .NamedCurves.ed25519
        case .ecdsa: oid = .NamedCurves.secp256k1
        }

        return .init(oid: oid)
    }

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(bytes: bytes)
    }

    public static func fromBytesEd25519(_ bytes: Data) throws -> Self {
        try Self(ed25519Bytes: bytes)
    }

    public static func fromBytesEcdsa(_ bytes: Data) throws -> Self {
        try Self(ecdsaBytes: bytes)
    }

    public static func fromBytesDer(_ bytes: Data) throws -> Self {
        let info: Pkcs8.PrivateKeyInfo
        let inner: ASN1OctetString
        do {
            info = try .init(derEncoded: Array(bytes))

            // PrivateKey is an `OctetString`, and the `PrivateKey`s we all support are `OctetStrings`.
            // So, we, awkwardly, have an `OctetString` containing an `OctetString` containing our key material.
            inner = try .init(derEncoded: info.privateKey.bytes)
        } catch {
            throw HError.keyParse(String(describing: error))
        }

        switch info.algorithm.oid {
        case .NamedCurves.ed25519: return try .fromBytesEd25519(Data(inner.bytes))
        case .NamedCurves.secp256k1: return try .fromBytesEcdsa(Data(inner.bytes))
        case let oid:
            throw HError.keyParse("unsupported key algorithm: \(oid)")
        }
    }

    private init<S: StringProtocol>(parsing description: S) throws {
        try self.init(bytes: Self.decodeBytes(description))
    }

    public static func fromString<S: StringProtocol>(_ description: S) throws -> Self {
        try Self(parsing: description)
    }

    public init?(_ description: String) {
        try? self.init(parsing: description)
    }

    public init(stringLiteral value: StringLiteralType) {
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
        let document = try Crypto.Pem.decode(pem)

        guard document.typeLabel == "PRIVATE KEY" else {
            throw HError.keyParse("incorrect PEM type label: expected: `PRIVATE KEY`, got: `\(document.typeLabel)`")
        }

        return try fromBytesDer(document.der)
    }

    /// Parse a `PrivateKey` from a password protected [PEM](https://www.rfc-editor.org/rfc/rfc7468#section-11) encoded string.
    public static func fromPem(_ pem: String, _ password: String) throws -> Self {
        let document = try Crypto.Pem.decode(pem)

        guard document.typeLabel == "ENCRYPTED PRIVATE KEY" else {
            throw HError.keyParse(
                "incorrect PEM type label: expected: `ENCRYPTED PRIVATE KEY`, got: `\(document.typeLabel)`")
        }

        let decrypted: Data

        do {
            let document = try Pkcs8.EncryptedPrivateKeyInfo(derEncoded: Array(document.der))
            decrypted = try document.decrypt(password: password.data(using: .utf8)!)
        } catch {
            throw HError.keyParse(String(describing: error))
        }

        return try .fromBytesDer(decrypted)

    }

    public func toBytesDer() -> Data {
        let rawBytes = Array(toBytesRaw())
        let inner: [UInt8]
        do {
            var serializer = DER.Serializer()

            // swiftlint:disable:next force_try
            try! serializer.serialize(ASN1OctetString(contentBytes: rawBytes[...]))

            inner = serializer.serializedBytes
        }

        let info = Pkcs8.PrivateKeyInfo(algorithm: algorithm, privateKey: .init(contentBytes: inner[...]))

        var serializer = DER.Serializer()

        // swiftlint:disable:next force_try
        try! serializer.serialize(info)
        return Data(serializer.serializedBytes)
    }

    public func toBytes() -> Data {
        switch kind {
        case .ed25519: return toBytesRaw()
        case .ecdsa: return toBytesDer()
        }
    }

    public func toBytesRaw() -> Data {
        switch kind {
        case .ecdsa(let ecdsa): return ecdsa.rawRepresentation
        case .ed25519(let ed25519): return ed25519.rawRepresentation
        }
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
        if case .ed25519 = kind {
            return true
        }

        return false
    }

    public func isEcdsa() -> Bool {
        if case .ecdsa = kind {
            return true
        }

        return false
    }

    public func sign(_ message: Data) -> Data {
        switch kind {
        case .ecdsa(let key):
            return try! key.ecdsa.signature(for: Keccak256Digest(Crypto.Sha3.keccak256(message))!).compactRepresentation
        case .ed25519(let key):
            return try! key.signature(for: message)
        }
    }

    public func isDerivable() -> Bool {
        isEd25519() && chainCode != nil
    }

    public func derive(_ index: Int32) throws -> Self {
        let hardenedMask: UInt32 = 1 << 31
        let index = UInt32(bitPattern: index)

        guard let chainCode = chainCode else {
            throw HError(kind: .keyDerive, description: "key is underivable")
        }

        switch kind {
        case .ecdsa: throw HError(kind: .keyDerive, description: "ecdsa keys are currently underivable")
        case .ed25519(let key):
            let index = index | hardenedMask

            var hmac = CryptoKit.HMAC<CryptoKit.SHA512>(key: .init(data: chainCode.data))

            hmac.update(data: [0])
            hmac.update(data: key.rawRepresentation)
            hmac.update(data: index.bigEndianBytes)
            let output = hmac.finalize().bytes

            let (data, chainCode) = (output[..<32], output[32...])

            return Self(
                kind: .ed25519(try! .init(rawRepresentation: data)),
                chainCode: Data(chainCode)
            )
        }
    }

    public func legacyDerive(_ index: Int64) throws -> Self {
        switch kind {
        case .ecdsa: throw HError(kind: .keyDerive, description: "ecdsa keys are currently underivable")

        case .ed25519(let key):
            var seed = key.rawRepresentation

            let i1: Int32
            switch index {
            case 0x00ff_ffff_ffff: i1 = 0xff
            case 0...: i1 = 0
            default: i1 = -1
            }

            let i2 = UInt8(truncatingIfNeeded: index)

            seed.append(i1.bigEndianBytes)
            seed.append(Data([i2, i2, i2, i2]))

            let salt = Data([0xff])

            let key = Pkcs5.pbkdf2(variant: .sha2(.sha512), password: seed, salt: salt, rounds: 2048, keySize: 32)

            // note: this shouldn't fail, but there isn't an infaliable conversion.
            return try .fromBytesEd25519(key)
        }
    }

    public static func fromMnemonic(_ mnemonic: Mnemonic, _ passphrase: String) -> Self {
        let seed = mnemonic.toSeed(passphrase: passphrase)

        var hmac = HMAC<SHA512>(key: .init(data: "ed25519 seed".data(using: .utf8)!))

        hmac.update(data: seed)

        let output = hmac.finalize().bytes

        let (data, chainCode) = (output[..<32], output[32...])

        var key = Self(
            kind: .ed25519(try! .init(rawRepresentation: data)),
            chainCode: Data(chainCode)
        )

        for index: Int32 in [44, 3030, 0, 0] {
            // an error here would be... Really weird because we just set chainCode.
            // swiftlint:disable:next force_try
            key = try! key.derive(index)
        }

        return key
    }

    public static func fromMnemonic(_ mnemonic: Mnemonic) -> Self {
        Self.fromMnemonic(mnemonic, "")
    }

    public func signTransaction(_ transaction: Transaction) throws {
        try transaction.freeze()

        transaction.addSignatureSigner(.privateKey(self))
    }
}

// for testing purposes :/
extension PrivateKey {
    internal func withChainCode(chainCode: Data) -> Self {
        precondition(chainCode.count == 32)
        return Self(kind: kind, chainCode: chainCode)
    }

    internal func prettyPrint() -> String {
        let data = toStringRaw()
        let chainCode = String(describing: chainCode?.data.hexStringEncoded())

        let start: String

        switch guts {
        case .ecdsa:
            start = "PrivateKey.ecdsa"
        case .ed25519:
            start = "PrivateKey.ed25519"
        }

        return """
            \(start)(
                key: \(data),
                chainCode: \(chainCode)
            )
            """
    }
}

#if compiler(>=5.7)
    extension PrivateKey.Repr: Sendable {}
#else
    extension PrivateKey.Repr: @unchecked Sendable {}
#endif

extension PrivateKey: Sendable {}
