import Foundation
import NumberKit

extension BigInt {
    internal init(unsignedBEBytes bytes: Data) {
        self.init(0)

        // yes, this is a stupid algorithm, but it's all I've got.
        // for a number like: 0x3d31_7dd0
        // `self` follows the pattern:
        // 0x00 (pre loop)
        // 0x00 -> 0x3d (first iteration)
        // 0x3d00 -> 0x3d31 (2nd)
        // 0x3d_3100 -> 0x3d_317d (3rd)
        // 0x3d31_7d00 -> 0x3d_7dd0 (4th)
        for byte in bytes {
            self <<= 8
            self += Self(byte)
        }
    }

    internal init(signedBEBytes bytes: Data) {
        self.init(0)

        // this algorithm is even more stupid than the previous one!
        // for a number like: 0x82c0_ffee
        // `self` follows the pattern:
        // 0x00 (0 - pre everything)
        // 0x00 -> 0x82 (-126 - first byte)
        // 0x8200 -> 0x82c0 (-32064 - first iteration)
        // 0x82_c000 -> 0x82_c0ff (2nd iteration)
        // 0x82c0_ff00 -> 0x82c0_ffee (-2101280786 - 3rd iteration)
        if let byte = bytes.first {
            self += Self(Int8(bitPattern: byte))
        }

        for byte in bytes.dropFirst() {
            self <<= 8
            self += Self(byte)
        }
    }

    internal func toBigEndianBytes() -> Data {
        Data(words.reversed().flatMap { $0.bigEndianBytes })
    }

    internal func toLittleEndianBytes() -> Data {
        Data(words.flatMap { $0.littleEndianBytes })
    }
}
