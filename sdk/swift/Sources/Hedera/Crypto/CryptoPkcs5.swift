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
import CommonCrypto
import CryptoKit
import Foundation
import SwiftASN1

internal enum Pkcs5 {}

extension Pkcs5 {
    internal static func pbkdf2(
        variant: Crypto.Hmac,
        password: Data,
        salt: Data,
        rounds: UInt32,
        keySize: Int
    ) -> Data {

        let prf: CCPBKDFAlgorithm
        switch variant {
        case .sha2(.sha256): prf = CCPBKDFAlgorithm(kCCPRFHmacAlgSHA256)
        case .sha2(.sha384): prf = CCPBKDFAlgorithm(kCCPRFHmacAlgSHA384)
        case .sha2(.sha512): prf = CCPBKDFAlgorithm(kCCPRFHmacAlgSHA512)
        }

        var derivedKey = Data(repeating: 0, count: keySize)

        let status = derivedKey.withUnsafeMutableTypedBytes { derivedKey in
            password.withUnsafeTypedBytes { password in
                salt.withUnsafeTypedBytes { salt in
                    CCKeyDerivationPBKDF(
                        CCPBKDFAlgorithm(kCCPBKDF2),
                        password.baseAddress,
                        password.count,
                        salt.baseAddress,
                        salt.count,
                        prf,
                        rounds,
                        derivedKey.baseAddress,
                        derivedKey.count
                    )
                }
            }
        }

        // an error here should be unreachable when it comes to hmac.
        precondition(status == 0, "pbkdf2 hmac failed with status: \(status)")

        return derivedKey
    }
}

extension Pkcs5 {
    /// RFC 5280 algorithm identifier.
    ///
    /// ```text
    ///    AlgorithmIdentifier  ::=  SEQUENCE  {
    ///    algorithm               OBJECT IDENTIFIER,
    ///    parameters              ANY DEFINED BY algorithm OPTIONAL  }
    /// ```
    internal struct AlgorithmIdentifier {
        internal init(oid: ASN1ObjectIdentifier, parameters: ASN1Any? = nil) {
            self.oid = oid
            self.parameters = parameters
        }

        internal let oid: ASN1ObjectIdentifier
        internal let parameters: ASN1Any?
    }
}

extension Pkcs5.AlgorithmIdentifier: SwiftASN1.DERImplicitlyTaggable {
    internal static var defaultIdentifier: ASN1Identifier {
        .sequence
    }

    internal init(derEncoded: ASN1Node, withIdentifier identifier: ASN1Identifier) throws {
        self = try DER.sequence(derEncoded, identifier: identifier) { nodes in
            let oid = try ASN1ObjectIdentifier(derEncoded: &nodes)
            let parameters = nodes.next().map(ASN1Any.init(derEncoded:))

            return Self(oid: oid, parameters: parameters)
        }
    }

    internal func serialize(into coder: inout DER.Serializer, withIdentifier identifier: ASN1Identifier) throws {
        try coder.appendConstructedNode(identifier: identifier) { coder in
            try coder.serialize(oid)

            if let parameters = parameters {
                try coder.serialize(parameters)
            }
        }
    }
}

extension ASN1ObjectIdentifier.NamedCurves {
    internal static let secp256k1: ASN1ObjectIdentifier = [1, 3, 132, 0, 10]
    internal static let ed25519: ASN1ObjectIdentifier = [1, 3, 101, 112]
}
