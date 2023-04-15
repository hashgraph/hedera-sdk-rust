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

import SwiftASN1

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
