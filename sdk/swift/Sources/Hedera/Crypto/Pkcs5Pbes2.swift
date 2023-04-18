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

extension Pkcs5 {
    /// ```text
    /// PBES2-params ::= SEQUENCE {
    ///  keyDerivationFunc AlgorithmIdentifier {{PBES2-KDFs}},
    ///  encryptionScheme AlgorithmIdentifier {{PBES2-Encs}} }
    /// ```
    internal struct Pbes2Parameters {
        internal let kdf: Pbes2Kdf
        internal let encryptionScheme: Pbes2EncryptionScheme
    }

    internal enum Pbes2Kdf {
        case pbkdf2(Pbkdf2Parameters)
        // todo: support scrypt keys?
        // case scrypt(ScryptParams)
    }

    internal enum Pbes2EncryptionScheme {
        /// >    The parameters field associated with this OID in an
        /// AlgorithmIdentifier shall have type OCTET STRING (SIZE(16)),
        /// specifying the initialization vector for CBC mode.
        /// ```text
        /// {OCTET STRING (SIZE(16)) IDENTIFIED BY aes128-CBC-PAD}
        /// ```
        case aes128Cbc(Data)
    }
}

extension Pkcs5.Pbes2Kdf: DERImplicitlyTaggable {
    internal static var defaultIdentifier: SwiftASN1.ASN1Identifier {
        Pkcs5.AlgorithmIdentifier.defaultIdentifier
    }

    internal init(derEncoded: ASN1Node, withIdentifier identifier: ASN1Identifier) throws {
        let algId = try Pkcs5.AlgorithmIdentifier(derEncoded: derEncoded, withIdentifier: identifier)

        guard let params = algId.parameters else {
            throw ASN1Error.invalidASN1Object
        }

        switch algId.oid {
        case .AlgorithmIdentifier.pbkdf2:
            self = .pbkdf2(try Pkcs5.Pbkdf2Parameters(asn1Any: params))
        default:
            throw ASN1Error.invalidASN1Object
        }
    }

    internal func serialize(into coder: inout DER.Serializer, withIdentifier identifier: ASN1Identifier) throws {
        let algId: Pkcs5.AlgorithmIdentifier
        switch self {
        case .pbkdf2(let params):
            algId = .init(oid: .AlgorithmIdentifier.pbkdf2, parameters: try .init(erasing: params))
        }

        try algId.serialize(into: &coder, withIdentifier: identifier)
    }
}

extension Pkcs5.Pbes2Kdf {
    internal func derive(password: Data, keySize: Int) throws -> Data {
        switch self {
        case .pbkdf2(let kdf):
            return try kdf.derive(password: password, keySize: keySize)
        }
    }
}

extension Pkcs5.Pbes2EncryptionScheme: DERImplicitlyTaggable {
    internal static var defaultIdentifier: SwiftASN1.ASN1Identifier {
        Pkcs5.AlgorithmIdentifier.defaultIdentifier
    }

    internal init(derEncoded: ASN1Node, withIdentifier identifier: ASN1Identifier) throws {
        let algId = try Pkcs5.AlgorithmIdentifier(derEncoded: derEncoded, withIdentifier: identifier)

        guard let params = algId.parameters else {
            throw ASN1Error.invalidASN1Object
        }

        switch algId.oid {
        case .AlgorithmIdentifier.aes128CbcPad:
            let params = try ASN1OctetString(asn1Any: params)
            guard params.bytes.count == 16 else {
                throw ASN1Error.invalidASN1Object
            }

            self = .aes128Cbc(Data(params.bytes))
        default:
            throw ASN1Error.invalidASN1Object
        }
    }

    internal func serialize(into coder: inout DER.Serializer, withIdentifier identifier: ASN1Identifier) throws {
        let algId: Pkcs5.AlgorithmIdentifier

        switch self {
        case .aes128Cbc(let params):
            algId = .init(
                oid: .AlgorithmIdentifier.aes128CbcPad,
                parameters: try .init(erasing: ASN1OctetString(contentBytes: Array(params)[...]))
            )
        }

        try algId.serialize(into: &coder, withIdentifier: identifier)
    }
}

extension Pkcs5.Pbes2EncryptionScheme {
    internal var keySize: Int {
        switch self {
        case .aes128Cbc:
            return 16
        }
    }

    internal func decrypt(key: Data, document: Data) throws -> Data {
        switch self {
        // note: the 128 in *this* is referring to the key size.
        case .aes128Cbc(let iv):
            return try Crypto.Aes.aes128CbcPadDecrypt(key: key, iv: iv, message: document)
        }
    }
}

extension Pkcs5.Pbes2Parameters: DERImplicitlyTaggable {
    internal static var defaultIdentifier: ASN1Identifier {
        .sequence
    }

    internal init(derEncoded: ASN1Node, withIdentifier identifier: ASN1Identifier) throws {
        self = try DER.sequence(derEncoded, identifier: identifier) { nodes in
            Self(
                kdf: try .init(derEncoded: &nodes),
                encryptionScheme: try .init(derEncoded: &nodes)
            )
        }
    }

    internal func serialize(into coder: inout DER.Serializer, withIdentifier identifier: ASN1Identifier) throws {
        try coder.appendConstructedNode(identifier: identifier) { coder in
            try kdf.serialize(into: &coder)
            try encryptionScheme.serialize(into: &coder)
        }
    }
}

extension Pkcs5.Pbes2Parameters {
    internal func decrypt(password: Data, document: Data) throws -> Data {
        let derivedKey = try kdf.derive(password: password, keySize: encryptionScheme.keySize)
        return try encryptionScheme.decrypt(key: derivedKey, document: document)

    }
}
