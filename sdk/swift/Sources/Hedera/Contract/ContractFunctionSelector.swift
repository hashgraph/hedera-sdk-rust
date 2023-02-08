/*
 * ‌
 * Hedera Swift SDK
 * ​
 * Copyright (C) 2023 - 2023 Hedera Hashgraph, LLC
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

import CryptoSwift
import Foundation

// note: this is a class in order to enable the builder pattern.
/// Builder for Solidity function selectors.
public final class ContractFunctionSelector {
    private var state: ContractFunctionSelectorState

    internal init(_ data: Data) {
        precondition(data.count == 4)

        state = .finished(data)
    }

    /// Create a selector for with the given function name.
    public init(_ functionName: String) {
        var hasher = CryptoSwift.SHA3(variant: .keccak256)
        // this never fails lol
        _ = try! hasher.update(withBytes: functionName.data(using: .utf8)!.bytes)
        _ = try! hasher.update(withBytes: "(".bytes)

        self.state = .building(hasher: hasher, needsComma: false)

    }

    @discardableResult
    internal func addParamType(_ solidityTypeName: String) -> Self {
        switch state {
        case .building(var hasher, var needsComma):
            if needsComma {
               _ =  try! hasher.update(withBytes: ",".bytes)
            }

            _ = try! hasher.update(withBytes: solidityTypeName.bytes)
            needsComma = true
            self.state = .building(hasher: hasher, needsComma: needsComma)
        case .finished:
            fatalError("Cannot add `\(solidityTypeName)` to finished `ContractFunctionSelector`")
        }

        return self
    }

    /// Finish creating the selector.
    ///
    /// Once this method is called the only valid interactions with `self` are to call `finish` again,
    /// or to use `output`.
    public func finish() -> Data {
        switch state {
        case .building(var hasher, _):
            _ = try! hasher.update(withBytes: ")".bytes)
            let output = try! Data(hasher.finish()[0..<4])
            state = .finished(output)
            return output
        case .finished(let data):
            return data
        }
    }

    /// If ``finish`` has been called, this will return the selector created, otherwise, this will return `nil`.
    public var output: Data? {
        switch state {
        case .building:
            return nil
        case .finished(let data):
            return data
        }
    }

    /// Add a solidity `function` to the function selector.
    @discardableResult
    public func addFunction() -> Self {
        addParamType("function")
    }

    /// Add a solidity `bool` to the function selector.
    @discardableResult
    public func addBool() -> Self {
        addParamType("bool")
    }

    /// Add a solidity `string` to the function selector.
    @discardableResult
    public func addString() -> Self {
        addParamType("string")
    }

    /// Add a solidity `string[]` to the function selector.
    @discardableResult
    public func addStringArray() -> Self {
        addParamType("string[]")
    }

    /// Add a solidity `bytes` to the function selector.
    @discardableResult
    public func addBytes() -> Self {
        addParamType("bytes")
    }

    /// Add a solidity `bytes[]` to the function selector.
    @discardableResult
    public func addBytesArray() -> Self {
        addParamType("bytes[]")
    }

    /// Add a solidity `bytes32` to the function selector.
    @discardableResult
    public func addBytes32() -> Self {
        addParamType("bytes32")
    }

    /// Add a solidity `bytes32[]` to the function selector.
    @discardableResult
    public func addBytes32Array() -> Self {
        addParamType("bytes32[]")
    }

    /// Add a solidity `int8` to the function selector.
    @discardableResult
    public func addInt8() -> Self {
        addParamType("int8")
    }

    /// Add a solidity `int8[]` to the function selector.
    @discardableResult
    public func addInt8Array() -> Self {
        addParamType("int8[]")
    }

    /// Add a solidity `int16` to the function selector.
    @discardableResult
    public func addInt16() -> Self {
        addParamType("int16")
    }

    /// Add a solidity `int16[]` to the function selector.
    @discardableResult
    public func addInt16Array() -> Self {
        addParamType("int16[]")
    }

    /// Add a solidity `int32` to the function selector.
    @discardableResult
    public func addInt32() -> Self {
        addParamType("int32")
    }

    /// Add a solidity `int32[]` to the function selector.
    @discardableResult
    public func addInt32Array() -> Self {
        addParamType("int32[]")
    }

    /// Add a solidity `int64` to the function selector.
    @discardableResult
    public func addInt64() -> Self {
        addParamType("int64")
    }

    /// Add a solidity `int64[]` to the function selector.
    @discardableResult
    public func addInt64Array() -> Self {
        addParamType("int64[]")
    }

    /// Add a solidity `int256` to the function selector.
    @discardableResult
    public func addInt256() -> Self {
        addParamType("int256")
    }

    /// Add a solidity `int256[]` to the function selector.
    @discardableResult
    public func addInt256Array() -> Self {
        addParamType("int256[]")
    }

    /// Add a solidity `uint8` to the function selector.
    @discardableResult
    public func addUint8() -> Self {
        addParamType("uint8")
    }

    /// Add a solidity `uint8[]` to the function selector.
    @discardableResult
    public func addUint8Array() -> Self {
        addParamType("uint8[]")
    }

    /// Add a solidity `uint16` to the function selector.
    @discardableResult
    public func addUint16() -> Self {
        addParamType("uint16")
    }

    /// Add a solidity `uint16[]` to the function selector.
    @discardableResult
    public func addUint16Array() -> Self {
        addParamType("uint16[]")
    }

    /// Add a solidity `uint32` to the function selector.
    @discardableResult
    public func addUint32() -> Self {
        addParamType("uint32")
    }

    /// Add a solidity `uint32[]` to the function selector.
    @discardableResult
    public func addUint32Array() -> Self {
        addParamType("uint32[]")
    }

    /// Add a solidity `uint64` to the function selector.
    @discardableResult
    public func addUint64() -> Self {
        addParamType("uint64")
    }

    /// Add a solidity `uint64[]` to the function selector.
    @discardableResult
    public func addUint64Array() -> Self {
        addParamType("uint64[]")
    }

    /// Add a solidity `uint256` to the function selector.
    @discardableResult
    public func addUint256() -> Self {
        addParamType("uint256")
    }

    /// Add a solidity `uint256[]` to the function selector.
    @discardableResult
    public func addUint256Array() -> Self {
        addParamType("uint256[]")
    }

    /// Add a solidity `address` to the function selector.
    @discardableResult
    public func addAddress() -> Self {
        addParamType("address")
    }

    /// Add a solidity `address[]` to the function selector.
    @discardableResult
    public func addAddressArray() -> Self {
        addParamType("address[]")
    }
}

private enum ContractFunctionSelectorState {
    case building(hasher: CryptoSwift.SHA3, needsComma: Bool)
    case finished(Data)
}
