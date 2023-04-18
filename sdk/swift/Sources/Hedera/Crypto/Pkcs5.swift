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

internal enum Pkcs5 {}

extension Pkcs5 {
    internal enum EncryptionScheme {
        case pbes2(Pbes2Parameters)
    }
}

extension ASN1ObjectIdentifier.NamedCurves {
    /// OID for the secp256k1 named curve.
    ///
    /// `iso(1) identified-organization(3) certicom(132) curve(0) 10`
    internal static let secp256k1: ASN1ObjectIdentifier = [1, 3, 132, 0, 10]
    // fixme: is this supposed to be *here*? It doesn't reaaally matter but...
    /// OID
    internal static let ed25519: ASN1ObjectIdentifier = [1, 3, 101, 112]
}

extension ASN1ObjectIdentifier.AlgorithmIdentifier {
    internal static let pbkdf2: ASN1ObjectIdentifier = [1, 2, 840, 113_549, 1, 5, 12]
    internal static let pbes2: ASN1ObjectIdentifier = [1, 2, 840, 113_549, 1, 5, 13]
    internal static let aes128CbcPad: ASN1ObjectIdentifier = [2, 16, 840, 1, 101, 3, 4, 1, 2]
}

extension Pkcs5.EncryptionScheme {
    internal func decrypt(password: Data, document: Data) throws -> Data {
        switch self {
        case .pbes2(let params): return try params.decrypt(password: password, document: document)
        }
    }
}

extension Pkcs5.EncryptionScheme: DERImplicitlyTaggable {
    internal static var defaultIdentifier: ASN1Identifier {
        Pkcs5.AlgorithmIdentifier.defaultIdentifier
    }

    internal init(derEncoded: ASN1Node, withIdentifier identifier: ASN1Identifier) throws {
        let algId = try Pkcs5.AlgorithmIdentifier(derEncoded: derEncoded, withIdentifier: identifier)

        guard let parameters = algId.parameters else {
            throw ASN1Error.invalidASN1Object
        }

        switch algId.oid {
        case .AlgorithmIdentifier.pbes2:
            self = .pbes2(try Pkcs5.Pbes2Parameters(asn1Any: parameters))
        default:
            throw ASN1Error.invalidASN1Object
        }
    }

    internal func serialize(into coder: inout DER.Serializer, withIdentifier identifier: ASN1Identifier) throws {
        let params: ASN1Any
        switch self {
        case .pbes2(let pbes2):
            params = try .init(erasing: pbes2)
        }

        return try Pkcs5.AlgorithmIdentifier(oid: .AlgorithmIdentifier.pbes2, parameters: params)
            .serialize(into: &coder, withIdentifier: identifier)
    }
}
