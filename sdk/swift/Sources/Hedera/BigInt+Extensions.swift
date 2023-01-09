import Foundation
import NumberKit

extension BigInt {
    internal func toBigEndianBytes() -> Data {
        Data(words.reversed().flatMap { $0.bigEndianBytes })
    }

    internal func toLittleEndianBytes() -> Data {
        Data(words.flatMap { $0.littleEndianBytes })
    }
}
