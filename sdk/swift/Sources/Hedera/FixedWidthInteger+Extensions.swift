import Foundation

extension FixedWidthInteger {
    internal init?(littleEndianBytes bytes: Data) {
        let size = MemoryLayout<Self>.size

        guard bytes.count == size else {
            return nil
        }

        self = 0

        _ = withUnsafeMutableBytes(of: &self, bytes.copyBytes(to:))

        self = littleEndian

    }

    internal init?(nativeEndianBytes bytes: Data) {
        let size = MemoryLayout<Self>.size

        guard bytes.count == size else {
            return nil
        }

        self = 0

        _ = withUnsafeMutableBytes(of: &self, bytes.copyBytes(to:))
    }

    internal init?(bigEndianBytes bytes: Data) {
        let size = MemoryLayout<Self>.size

        guard bytes.count == size else {
            return nil
        }

        self = 0

        _ = withUnsafeMutableBytes(of: &self, bytes.copyBytes(to:))

        self = bigEndian

    }

    internal var nativeEndianBytes: Data {
        var num: Self = self
        return Data(bytes: &num, count: MemoryLayout.size(ofValue: num))
    }

    internal var littleEndianBytes: Data {
        var num: Self = self.littleEndian
        return Data(bytes: &num, count: MemoryLayout.size(ofValue: num))
    }

    internal var bigEndianBytes: Data {
        var num: Self = self.bigEndian
        return Data(bytes: &num, count: MemoryLayout.size(ofValue: num))
    }
}

extension FixedWidthInteger {
    internal init<S: StringProtocol>(parsing description: S) throws {
        guard let value = Self(description) else {
            throw HError.basicParse("Invalid numeric string `\(description)`")
        }

        self = value
    }
}
