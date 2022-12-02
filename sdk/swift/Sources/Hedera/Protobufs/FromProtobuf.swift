import Foundation
import SwiftProtobuf

internal protocol TryFromProtobuf {
    associatedtype Protobuf

    init(fromProtobuf proto: Protobuf) throws
}

extension TryFromProtobuf {
    internal static func fromProtobuf(_ proto: Protobuf) throws -> Self {
        try Self(fromProtobuf: proto)
    }

    internal init(fromProtobufBytes bytes: Data) throws where Protobuf: SwiftProtobuf.Message {
        try self.init(fromProtobuf: try Protobuf(contiguousBytes: bytes))
    }
}

// Swift is really weird and lets you do this but doesn't let you do a `rethrows` style impl.
internal protocol FromProtobuf: TryFromProtobuf {
    init(fromProtobuf proto: Protobuf)
}

extension FromProtobuf {
    internal static func fromProtobuf(_ proto: Protobuf) -> Self {
        Self(fromProtobuf: proto)
    }
}

extension Optional: TryFromProtobuf where Wrapped: TryFromProtobuf {
    internal typealias Protobuf = Wrapped.Protobuf?

    internal init(fromProtobuf proto: Wrapped.Protobuf?) throws {
        self = try proto.map(Wrapped.fromProtobuf)
    }
}

extension Optional: FromProtobuf where Wrapped: FromProtobuf {
    internal typealias Protobuf = Wrapped.Protobuf?

    internal init(fromProtobuf proto: Wrapped.Protobuf?) {
        self = proto.map(Wrapped.fromProtobuf)
    }
}

extension Array: TryFromProtobuf where Element: TryFromProtobuf {
    internal typealias Protobuf = [Element.Protobuf]

    internal init(fromProtobuf proto: [Element.Protobuf]) throws {
        self = try proto.map(Element.fromProtobuf)
    }
}

extension Array: FromProtobuf where Element: FromProtobuf {
    internal typealias Protobuf = [Element.Protobuf]

    internal init(fromProtobuf proto: [Element.Protobuf]) {
        self = proto.map(Element.fromProtobuf)
    }
}
