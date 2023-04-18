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

import Foundation
import SwiftASN1

internal enum Pkcs8 {
    internal typealias PrivateKeyAlgorithmIdentifier = Pkcs5.AlgorithmIdentifier

    /// Content varies based on type of key.
    /// The algorithm identifier dictates the format of the key.
    /// ```text
    /// PrivateKey ::= OCTET STRING
    /// ```
    internal typealias PrivateKey = ASN1OctetString

    /// Content varies based on type of key.
    /// The algorithm identifier dictates the format of the key.
    ///
    /// ```text
    /// PublicKey ::= BIT STRING
    /// ```
    internal typealias PublicKey = ASN1BitString

    /// EncryptedData ::= OCTET STRING
    internal typealias EncryptedData = ASN1OctetString

    /// ```text
    ///  Version ::= Integer { { v1(0), v2(1) } (v1, ..., v2) }
    /// ```
    fileprivate enum Version: Int, Equatable {
        case v1 = 0
        case v2 = 1
    }

    /// PCKS8 Private key info.
    ///
    /// When version is ``Version.v1``:
    ///
    /// ```text
    /// PrivateKeyInfo ::= SEQUENCE {
    ///    version                   Version,
    ///    privateKeyAlgorithm       PrivateKeyAlgorithmIdentifier,
    ///    privateKey                PrivateKey,
    ///    attributes           [0]  IMPLICIT Attributes OPTIONAL }
    /// ```
    ///
    /// When version is ``Version.v2``:
    ///
    /// ```text
    /// OneAsymmetricKey ::= SEQUENCE {
    ///   version                   Version,
    ///   privateKeyAlgorithm       PrivateKeyAlgorithmIdentifier,
    ///   privateKey                PrivateKey,
    ///   attributes            [0] Attributes OPTIONAL,
    ///   ...,
    ///   [[2: publicKey        [1] PublicKey OPTIONAL ]],
    ///   ...
    /// }
    /// PrivateKeyInfo ::= OneAsymmetricKey
    /// ```
    ///
    /// Attributes are currently unneeded (and so unsupported)
    internal struct PrivateKeyInfo {
        internal init(
            algorithm: Pkcs8.PrivateKeyAlgorithmIdentifier,
            privateKey: ASN1OctetString,
            publicKey: ASN1BitString? = nil
        ) {
            self.algorithm = algorithm
            self.privateKey = privateKey
            self.publicKey = publicKey
        }

        internal let algorithm: PrivateKeyAlgorithmIdentifier
        internal let privateKey: ASN1OctetString
        internal let publicKey: ASN1BitString?

        fileprivate var version: Version {
            publicKey != nil ? .v2 : .v1
        }
    }

    /// ```text
    /// SubjectPublicKeyInfo  ::=  SEQUENCE  {
    ///    algorithm            AlgorithmIdentifier,
    ///    subjectPublicKey     BIT STRING  }
    /// ```
    internal struct SubjectPublicKeyInfo {
        internal let algorithm: Pkcs5.AlgorithmIdentifier
        internal let subjectPublicKey: ASN1BitString
    }

    /// ```text
    /// EncryptedPrivateKeyInfo ::= SEQUENCE {
    ///   encryptionAlgorithm  EncryptionAlgorithmIdentifier,
    ///   encryptedData        EncryptedData }
    ///
    /// EncryptionAlgorithmIdentifier ::= AlgorithmIdentifier
    /// ```
    internal struct EncryptedPrivateKeyInfo {
        internal let encryptionAlgorithm: Pkcs5.EncryptionScheme
        internal let encryptedData: ASN1OctetString
    }
}

extension Pkcs8.Version: DERImplicitlyTaggable {
    fileprivate static var defaultIdentifier: ASN1Identifier {
        .integer
    }

    init(derEncoded: ASN1Node, withIdentifier identifier: ASN1Identifier) throws {
        let raw = try Int(derEncoded: derEncoded, withIdentifier: identifier)

        guard let value = Self(rawValue: raw) else {
            throw ASN1Error.invalidASN1Object
            // throw ASN1Error.invalidASN1Object(reason: "invalid Pkcs8.Version")
        }

        self = value
    }

    func serialize(into coder: inout DER.Serializer, withIdentifier identifier: ASN1Identifier)
        throws
    {
        try coder.serialize(self.rawValue)
    }
}

extension Pkcs8.PrivateKeyInfo: DERImplicitlyTaggable {
    private static let publicKeyTagNumber: UInt = 1

    internal static var defaultIdentifier: ASN1Identifier {
        .sequence
    }

    internal init(derEncoded rootNode: ASN1Node, withIdentifier identifier: ASN1Identifier) throws {
        // DER.sequence(node: ASN1Node, identifier: ASN1Identifier, builder: (inout ASN1NodeCollection.Iterator) throws -> T)
        self = try DER.sequence(rootNode, identifier: identifier) { nodes in
            let version = try Pkcs8.Version(derEncoded: &nodes)
            let algorithmIdentifier = try Pkcs8.PrivateKeyAlgorithmIdentifier(derEncoded: &nodes)
            let privateKey = try Pkcs8.PrivateKey(derEncoded: &nodes)
            let publicKey = try DER.optionalExplicitlyTagged(
                &nodes,
                tagNumber: Self.publicKeyTagNumber,
                tagClass: .contextSpecific,
                Pkcs8.PublicKey.init(derEncoded:)
            )

            switch (version, publicKey != nil) {
            case (.v1, false), (.v2, true): break
            case (.v1, true), (.v2, false):
                throw ASN1Error.invalidASN1Object
            // throw ASN1Error.invalidASN1Object(reason: "invalid version, public key combo")
            }

            return Self(algorithm: algorithmIdentifier, privateKey: privateKey, publicKey: publicKey)
        }
    }

    internal func serialize(into coder: inout DER.Serializer, withIdentifier identifier: ASN1Identifier) throws {
        try coder.appendConstructedNode(identifier: identifier) { coder in
            try coder.serialize(version)
            try coder.serialize(algorithm)
            try coder.serialize(privateKey)

            if let publicKey = publicKey {
                try coder.serialize(
                    publicKey,
                    explicitlyTaggedWithTagNumber: Self.publicKeyTagNumber,
                    tagClass: .contextSpecific
                )
            }
        }
    }
}

extension Pkcs8.SubjectPublicKeyInfo: DERImplicitlyTaggable {
    internal static var defaultIdentifier: SwiftASN1.ASN1Identifier {
        .sequence
    }

    internal init(derEncoded: ASN1Node, withIdentifier identifier: ASN1Identifier) throws {
        self = try DER.sequence(derEncoded, identifier: identifier) { nodes in
            let algId = try Pkcs5.AlgorithmIdentifier(derEncoded: &nodes)
            let subjectPublicKey = try ASN1BitString(derEncoded: &nodes)

            return Self(algorithm: algId, subjectPublicKey: subjectPublicKey)
        }
    }

    internal func serialize(into coder: inout DER.Serializer, withIdentifier identifier: ASN1Identifier) throws {
        try coder.appendConstructedNode(identifier: identifier) { coder in
            try coder.serialize(algorithm)
            try coder.serialize(subjectPublicKey)
        }
    }
}

extension Pkcs8.EncryptedPrivateKeyInfo {
    internal func decrypt(password: Data) throws -> Data {
        try encryptionAlgorithm.decrypt(password: password, document: Data(encryptedData.bytes))
    }
}

extension Pkcs8.EncryptedPrivateKeyInfo: DERImplicitlyTaggable {
    internal static var defaultIdentifier: SwiftASN1.ASN1Identifier {
        .sequence
    }

    internal init(derEncoded: ASN1Node, withIdentifier identifier: ASN1Identifier) throws {
        self = try DER.sequence(derEncoded, identifier: identifier) { nodes in
            let encryptionAlgorithm = try Pkcs5.EncryptionScheme(derEncoded: &nodes)
            let encryptedData = try ASN1OctetString(derEncoded: &nodes)

            return Self(encryptionAlgorithm: encryptionAlgorithm, encryptedData: encryptedData)
        }
    }

    internal func serialize(into coder: inout DER.Serializer, withIdentifier identifier: ASN1Identifier) throws {
        try coder.appendConstructedNode(identifier: identifier) { coder in
            try coder.serialize(encryptionAlgorithm)
            try coder.serialize(encryptedData)
        }
    }
}
