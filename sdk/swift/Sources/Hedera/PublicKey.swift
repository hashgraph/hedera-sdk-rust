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

import CryptoKit
import Foundation
import HederaProtobufs
import SwiftASN1
import secp256k1
import secp256k1_bindings

/// A public key on the Hedera network.
public struct PublicKey: LosslessStringConvertible, ExpressibleByStringLiteral, Equatable, Hashable {
    // we need to be sendable, so...
    // The idea being that we initialize the key whenever we need it, which is absolutely not free, but it is `Sendable`.
    fileprivate enum Repr {
        case ed25519(Data)
        case ecdsa(Data, compressed: Bool)

        fileprivate init(kind: PublicKey.Kind) {
            switch kind {
            case .ecdsa(let key): self = .ecdsa(key.rawRepresentation, compressed: key.format == .compressed)
            case .ed25519(let key): self = .ed25519(key.rawRepresentation)
            }
        }

        fileprivate var kind: PublicKey.Kind {
            // swiftlint:disable force_try
            switch self {
            case .ecdsa(let key, let compressed):
                return .ecdsa(try! .init(rawRepresentation: key, format: compressed ? .compressed : .uncompressed))
            case .ed25519(let key): return .ed25519(try! .init(rawRepresentation: key))
            }

            // swiftlint:enable force_try
        }
    }

    fileprivate enum Kind {
        case ed25519(CryptoKit.Curve25519.Signing.PublicKey)
        case ecdsa(secp256k1.Signing.PublicKey)
    }

    private init(_ kind: Kind) {
        self.guts = .init(kind: kind)
    }

    private let guts: Repr

    private var kind: Kind {
        guts.kind
    }

    private static func decodeBytes<S: StringProtocol>(_ description: S) throws -> Data {
        let description = description.stripPrefix("0x") ?? description[...]
        guard let bytes = Data(hexEncoded: description) else {
            throw HError(kind: .keyParse, description: "Invalid hex string")
        }

        return bytes
    }

    internal static func ed25519(_ key: CryptoKit.Curve25519.Signing.PublicKey) -> Self {
        Self(.ed25519(key))
    }

    internal static func ecdsa(_ key: secp256k1.Signing.PublicKey) -> Self {
        Self(.ecdsa(key))
    }

    private init(bytes: Data) throws {
        switch bytes.count {
        case 32: try self.init(ed25519Bytes: bytes)
        case 33: try self.init(ecdsaBytes: bytes)
        default: try self.init(derBytes: bytes)
        }
    }

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(bytes: bytes)
    }

    fileprivate init(ed25519Bytes bytes: Data) throws {
        guard bytes.count == 32 else {
            try self.init(derBytes: bytes)
            return
        }

        do {
            self.init(.ed25519(try .init(rawRepresentation: bytes)))
        } catch {
            throw HError.keyParse(String(describing: error))
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

    public static func fromBytesEd25519(_ bytes: Data) throws -> Self {
        try Self(ed25519Bytes: bytes)
    }

    fileprivate init(ecdsaBytes bytes: Data) throws {
        guard bytes.count == 33 else {
            try self.init(derBytes: bytes)
            return
        }

        do {
            self.init(.ecdsa(try .init(rawRepresentation: bytes, format: .compressed)))
        } catch {
            throw HError.keyParse(String(describing: error))
        }

    }

    public static func fromBytesEcdsa(_ bytes: Data) throws -> Self {
        try Self(ecdsaBytes: bytes)
    }

    fileprivate init(derBytes bytes: Data) throws {
        let info: Pkcs8.SubjectPublicKeyInfo

        do {
            info = try .init(derEncoded: Array(bytes))
        } catch {
            throw HError.keyParse(String(describing: error))
        }

        switch info.algorithm.oid {
        // hack: Rust had a bug where it used the wrong OID for public keys, the only test verifying the correct behavior exists in JS,
        // rather than Java, where the impl was ported from, the incorrect OID being `id-ecpub`.
        case .NamedCurves.secp256k1, .AlgorithmIdentifier.idEcPublicKey:
            guard info.subjectPublicKey.paddingBits == 0 else {
                throw HError.keyParse("Invalid padding for secp256k1 spki")
            }

            try self.init(ecdsaBytes: Data(info.subjectPublicKey.bytes))

        case .NamedCurves.ed25519:
            guard info.subjectPublicKey.paddingBits == 0 else {
                throw HError.keyParse("Invalid padding for ed25519 spki")
            }

            try self.init(ed25519Bytes: Data(info.subjectPublicKey.bytes))

        default:
            throw HError.keyParse("Unknown public key OID \(info.algorithm.oid)")
        }
    }

    public static func fromBytesDer(_ bytes: Data) throws -> Self {
        try Self(derBytes: bytes)
    }

    internal static func fromAliasBytes(_ bytes: Data) throws -> PublicKey? {
        if bytes.isEmpty {
            return nil
        }

        switch try Key(protobufBytes: bytes) {
        case .single(let key):
            return key

        default:
            throw HError.fromProtobuf("Unexpected key kind in Account alias")
        }
    }

    private init(parsing description: String) throws {
        try self.init(bytes: Self.decodeBytes(description))
    }

    public static func fromString(_ description: String) throws -> Self {
        try Self(parsing: description)
    }

    public init?(_ description: String) {
        try? self.init(parsing: description)
    }

    public init(stringLiteral value: StringLiteralType) {
        // swiftlint:disable:next force_try
        try! self.init(parsing: value)
    }

    public static func fromStringDer(_ description: String) throws -> Self {
        try fromBytesDer(decodeBytes(description))
    }

    public static func fromStringEd25519(_ description: String) throws -> Self {
        try fromBytesEd25519(decodeBytes(description))
    }

    public static func fromStringEcdsa(_ description: String) throws -> Self {
        try fromBytesEcdsa(decodeBytes(description))
    }

    public func toBytesDer() -> Data {
        let spki = Pkcs8.SubjectPublicKeyInfo(
            algorithm: algorithm,
            subjectPublicKey: ASN1BitString(bytes: Array(toBytesRaw())[...])
        )

        var serializer = DER.Serializer()

        // swiftlint:disable:next force_try
        try! serializer.serialize(spki)
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
        case .ecdsa(let key): return key.rawRepresentation
        case .ed25519(let key): return key.rawRepresentation
        }
    }

    public var description: String {
        toBytesDer().hexStringEncoded()
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
        AccountId(shard: shard, realm: realm, alias: self)
    }

    public func verify(_ message: Data, _ signature: Data) throws {
        switch self.kind {
        case .ed25519(let key):
            guard key.isValidSignature(signature, for: message) else {
                throw HError(kind: .signatureVerify, description: "invalid signature")
            }

        case .ecdsa(let key):
            let isValid: Bool
            do {
                isValid = try key.ecdsa.isValidSignature(
                    .init(compactRepresentation: signature), for: Keccak256Digest(Crypto.Sha3.keccak256(message))!)
            } catch {
                throw HError(kind: .signatureVerify, description: "invalid signature")
            }

            guard isValid
            else {
                throw HError(kind: .signatureVerify, description: "invalid signature")
            }
        }
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

    public static func == (lhs: PublicKey, rhs: PublicKey) -> Bool {
        // this will always be true for public keys, DER is a stable format with canonicalization.
        // ideally we'd do this a different way, but that needs to wait until ffi is gone.
        lhs.toBytesDer() == rhs.toBytesDer()
    }

    public func hash(into hasher: inout Hasher) {
        hasher.combine(toBytesDer())
    }

    /// Convert this public key into an evm address. The EVM address is This is the rightmost 20 bytes of the 32 byte Keccak-256 hash of the ECDSA public key.
    public func toEvmAddress() -> EvmAddress? {
        guard case .ecdsa(let key) = self.kind else {
            return nil
        }

        // when the bindings aren't enough :/
        var pubkey = secp256k1_pubkey()

        key.rawRepresentation.withUnsafeTypedBytes { bytes in
            let result = secp256k1_bindings.secp256k1_ec_pubkey_parse(
                secp256k1.Context.raw,
                &pubkey,
                bytes.baseAddress!,
                bytes.count
            )

            precondition(result == 1)
        }

        var output = Data(repeating: 0, count: 65)

        output.withUnsafeMutableTypedBytes { output in
            var outputLen = output.count

            let result = secp256k1_ec_pubkey_serialize(
                secp256k1.Context.raw, output.baseAddress!,
                &outputLen,
                &pubkey,
                secp256k1.Format.uncompressed.rawValue
            )

            precondition(result == 1)
            precondition(outputLen == output.count)
        }

        // fixme(important): sec1 uncompressed point
        let hash = Crypto.Sha3.keccak256(output[1...])

        return try! EvmAddress(Data(hash.dropFirst(12)))

    }

    internal func verifyTransactionSources(_ sources: TransactionSources) throws {
        let pkBytes = self.toBytesRaw()

        for signedTransaction in sources.signedTransactions {
            var found = false

            for sigPair in signedTransaction.sigMap.sigPair
            where pkBytes.starts(with: sigPair.pubKeyPrefix) {
                found = true

                let signature: Data

                switch sigPair.signature {
                case .ecdsaSecp256K1(let data), .ed25519(let data): signature = data
                default: throw HError(kind: .signatureVerify, description: "Unsupported transaction signature type")
                }

                try verify(signedTransaction.bodyBytes, signature)
            }

            if !found {
                throw HError(kind: .signatureVerify, description: "signer not in transaction")
            }
        }

    }

    public func verifyTransaction(_ transaction: Transaction) throws {
        // we're a signer.
        if transaction.signers.contains(where: { self == $0.publicKey }) {
            return
        }

        guard let sources = transaction.sources else {
            throw HError(kind: .signatureVerify, description: "signer not in transaction")
        }

        try verifyTransactionSources(sources)
    }
}

extension PublicKey: TryProtobufCodable {
    internal typealias Protobuf = Proto_Key

    internal init(protobuf proto: Proto_Key) throws {
        guard let key = proto.key else {
            throw HError.fromProtobuf("Key protobuf kind was unexpectedly `nil`")
        }

        switch key {
        case .ed25519(let bytes):
            try self.init(ed25519Bytes: bytes)

        case .contractID:
            throw HError.fromProtobuf("unsupported Contract ID key in single key")

        case .delegatableContractID:
            throw HError.fromProtobuf("unsupported Delegatable Contract ID key in single key")

        case .rsa3072:
            throw HError.fromProtobuf("unsupported RSA-3072 key in single key")

        case .ecdsa384:
            throw HError.fromProtobuf("unsupported ECDSA-384 key in single key")

        case .thresholdKey:
            throw HError.fromProtobuf("unsupported threshold key in single key")

        case .keyList:
            throw HError.fromProtobuf("unsupported keylist in single key")

        case .ecdsaSecp256K1(let bytes):
            try self.init(ecdsaBytes: bytes)
        }
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            if self.isEd25519() {
                proto.ed25519 = toBytesRaw()
            } else if self.isEcdsa() {
                proto.ecdsaSecp256K1 = toBytesRaw()
            } else {
                fatalError("BUG: Unexpected PublicKey kind")
            }
        }
    }
}

#if compiler(>=5.7)
    extension PublicKey.Repr: Sendable {}
#else
    extension PublicKey.Repr: @unchecked Sendable {}
#endif

extension PublicKey: Sendable {}
