import Foundation

extension TimeInterval {
    internal init(wholeSeconds: UInt32) {
        self.init(wholeSeconds)
    }

    internal var wholeSeconds: UInt32 {
        UInt32(self)
    }
}
