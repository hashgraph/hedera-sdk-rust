import Foundation

extension TimeInterval {
    init(wholeSeconds: UInt32) {
        self.init(wholeSeconds)
    }

    var wholeSeconds: UInt32 {
        UInt32(self)
    }
}
