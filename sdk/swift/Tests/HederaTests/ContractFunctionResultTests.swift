import NumberKit
import XCTest

@testable import Hedera

extension ContractFunctionResult {
    internal static func sampleWithBytes(bytes: Data) -> Self {
        Self(
            contractId: 0,
            bloom: Data(),
            gasUsed: 0,
            gas: 0,
            hbarAmount: 0,
            contractFunctionParametersBytes: Data(),
            bytes: bytes
        )
    }
}

private func numToBytes32<T: FixedWidthInteger>(value: T) -> Data {
    return Data(repeating: 0, count: 32 - MemoryLayout<T>.size) + value.bigEndianBytes
}

private let callResult = Data(
    hexEncoded:
        """
        00000000000000000000000000000000000000000000000000000000ffffffff\
        7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        00000000000000000000000011223344556677889900aabbccddeeff00112233\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        00000000000000000000000000000000000000000000000000000000000000c0\
        0000000000000000000000000000000000000000000000000000000000000100\
        000000000000000000000000000000000000000000000000000000000000000d\
        48656c6c6f2c20776f726c642100000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000014\
        48656c6c6f2c20776f726c642c20616761696e21000000000000000000000000
        """
)!

private let stringArrayResult = Data(
    hexEncoded:
        """
        0000000000000000000000000000000000000000000000000000000000000020\
        0000000000000000000000000000000000000000000000000000000000000002\
        0000000000000000000000000000000000000000000000000000000000000040\
        0000000000000000000000000000000000000000000000000000000000000080\
        000000000000000000000000000000000000000000000000000000000000000C\
        72616E646F6D2062797465730000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000E\
        72616E646F6D2062797465732032000000000000000000000000000000000000
        """
)!

private let stringArrayResult2 = Data(
    hexEncoded:
        """
        0000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000060\
        0000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000002\
        0000000000000000000000000000000000000000000000000000000000000040\
        0000000000000000000000000000000000000000000000000000000000000080\
        000000000000000000000000000000000000000000000000000000000000000c\
        72616e646f6d206279746573000000000000000000000000c0ffee0000000000\
        000000000000000000000000000000000000000000000000000000000000000e\
        72616E646F6D2062797465732032000000000000decaff000000000000000000
        """
)!

public final class ContractFunctionResultTests: XCTestCase {
    public func testGetUInt8() {
        let result = ContractFunctionResult.sampleWithBytes(bytes: numToBytes32(value: UInt8(0xdc)))

        XCTAssertEqual(result.getUInt8(0), 0xdc)
    }

    public func testGetUInt32() {
        let result = ContractFunctionResult.sampleWithBytes(bytes: numToBytes32(value: UInt32(0x10de_caff)))

        XCTAssertEqual(result.getUInt32(0), 0x10de_caff)
    }

    public func testGetBytes() {
        let offset = numToBytes32(value: UInt32(32))
        let len = numToBytes32(value: UInt32(3))
        let data = Data([0xde, 0xca, 0xff])
        let bytes = offset + len + data

        let result: ContractFunctionResult = ContractFunctionResult.sampleWithBytes(bytes: bytes)

        XCTAssertEqual(result.getBytes(0)?.hexStringEncoded(), "decaff")
    }

    public func testProvidesResults() {
        let result = ContractFunctionResult.sampleWithBytes(bytes: callResult)

        XCTAssertEqual(result.getBool(0), true)
        XCTAssertEqual(result.getInt32(0), -1)
        XCTAssertEqual(result.getInt64(0), Int64(UInt32.max))
        XCTAssertEqual(result.getInt256(0), BigInt(UInt32.max))
        XCTAssertEqual(result.getInt256(1), (1 << 255) - 1)
        XCTAssertEqual(result.getAddress(2), "11223344556677889900aabbccddeeff00112233")
        XCTAssertEqual(result.getUInt32(3), .max)
        XCTAssertEqual(result.getUInt64(3), .max)
        // BigInteger can represent the full range and so should be 2^256 - 1
        XCTAssertEqual(result.getUInt256(3), (1 << 256) - 1)
        XCTAssertEqual(result.getInt256(3), -1)

        XCTAssertEqual(result.getString(4), "Hello, world!")
        XCTAssertEqual(result.getString(5), "Hello, world, again!")
    }

    public func testStringArrayResults() {
        let result = ContractFunctionResult.sampleWithBytes(bytes: stringArrayResult)

        guard let strings = result.getStringArray(0) else {
            XCTFail("string array caused index out of bounds")
            return
        }

        XCTAssertEqual(strings[safe: 0], "random bytes")
        XCTAssertEqual(strings[safe: 1], "random bytes 2")
        XCTAssertEqual(strings.count, 2)
    }

    public func testStringArrayResults2() {
        let result = ContractFunctionResult.sampleWithBytes(bytes: stringArrayResult2)

        guard let strings = result.getStringArray(1) else {
            XCTFail("string array caused index out of bounds")
            return
        }

        XCTAssertEqual(strings[safe: 0], "random bytes")
        XCTAssertEqual(strings[safe: 1], "random bytes 2")
        XCTAssertEqual(strings.count, 2)
    }

    public func testIndexOOBNoCrash() {
        let result = ContractFunctionResult.sampleWithBytes(bytes: Data())

        XCTAssertNil(result.getString((0)))
        XCTAssertNil(result.getStringArray((0)))
        XCTAssertNil(result.getBytes((0)))
        XCTAssertNil(result.getBytes32((0)))
        XCTAssertNil(result.getBool((0)))
        XCTAssertNil(result.getInt8((0)))
        XCTAssertNil(result.getUInt8((0)))
        XCTAssertNil(result.getInt32((0)))
        XCTAssertNil(result.getUInt32((0)))
        XCTAssertNil(result.getInt64((0)))
        XCTAssertNil(result.getUInt64((0)))
        XCTAssertNil(result.getInt256((0)))
        XCTAssertNil(result.getUInt256((0)))
        XCTAssertNil(result.getAddress((0)))
    }
}
