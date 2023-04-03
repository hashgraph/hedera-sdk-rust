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
import HederaProtobufs

// todo: deduplicate these with `PrivateKey.swift`

private typealias UnsafeFromBytesFunc = @convention(c) (
    UnsafePointer<UInt8>?, Int, UnsafeMutablePointer<OpaquePointer?>?
) -> HederaError

/// A public key on the Hedera network.
public final class PublicKey: LosslessStringConvertible, ExpressibleByStringLiteral, Equatable, Hashable {
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
        try self.init(bytes: bytes, unsafeCallback: hedera_public_key_from_bytes)
    }

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(bytes: bytes)
    }

    fileprivate convenience init(ed25519Bytes bytes: Data) throws {
        try self.init(bytes: bytes, unsafeCallback: hedera_public_key_from_bytes_ed25519)
    }

    public static func fromBytesEd25519(_ bytes: Data) throws -> Self {
        try Self(ed25519Bytes: bytes)
    }

    fileprivate convenience init(ecdsaBytes bytes: Data) throws {
        try self.init(bytes: bytes, unsafeCallback: hedera_public_key_from_bytes_ecdsa)
    }

    public static func fromBytesEcdsa(_ bytes: Data) throws -> Self {
        try Self(ecdsaBytes: bytes)
    }

    public static func fromBytesDer(_ bytes: Data) throws -> Self {
        try Self(bytes: bytes, unsafeCallback: hedera_public_key_from_bytes_der)
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

    private convenience init(parsing description: String) throws {
        try self.init(bytes: Self.decodeBytes(description))
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
        try fromBytesDer(decodeBytes(description))
    }

    public static func fromStringEd25519(_ description: String) throws -> Self {
        try fromBytesEd25519(decodeBytes(description))
    }

    public static func fromStringEcdsa(_ description: String) throws -> Self {
        try fromBytesEcdsa(decodeBytes(description))
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
        let dataPtr = hedera_public_key_to_evm_address(ptr)

        return dataPtr.map { dataPtr in
            // we literally write 20 as the count, which is the only requirement for `EvmAddress`.
            // swiftlint:disable:next force_try
            try! EvmAddress(Data(bytesNoCopy: dataPtr, count: 20, deallocator: .unsafeCHederaBytesFree))
        }
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

    deinit {
        hedera_public_key_free(ptr)
    }
}

extension PublicKey: TryProtobufCodable {
    internal typealias Protobuf = Proto_Key

    internal convenience init(protobuf proto: Proto_Key) throws {
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

extension PublicKey: Sendable {}
