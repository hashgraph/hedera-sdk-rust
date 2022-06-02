import Foundation

internal extension TimeInterval {
    init(wholeSeconds: UInt32) {
        self.init(wholeSeconds)
    }

    var wholeSeconds: UInt32 {
        get {
            UInt32(self)
        }
    }
}
