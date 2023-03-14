import Foundation
import SwiftProtobuf

internal protocol TryFromProtobuf {
    associatedtype Protobuf

    init(protobuf proto: Protobuf) throws
}

extension TryFromProtobuf {
    internal static func fromProtobuf(_ proto: Protobuf) throws -> Self {
        try Self(protobuf: proto)
    }

    internal init(protobufBytes bytes: Data) throws where Protobuf: SwiftProtobuf.Message {
        try self.init(protobuf: try Protobuf(contiguousBytes: bytes))
    }
}

// Swift is really weird and lets you do this but doesn't let you do a `rethrows` style impl.
internal protocol FromProtobuf: TryFromProtobuf {
    init(protobuf proto: Protobuf)
}

extension FromProtobuf {
    internal static func fromProtobuf(_ proto: Protobuf) -> Self {
        Self(protobuf: proto)
    }
}

extension Optional: TryFromProtobuf where Wrapped: TryFromProtobuf {
    internal typealias Protobuf = Wrapped.Protobuf?

    internal init(protobuf proto: Wrapped.Protobuf?) throws {
        self = try proto.map(Wrapped.fromProtobuf)
    }
}

extension Optional: FromProtobuf where Wrapped: FromProtobuf {
    internal typealias Protobuf = Wrapped.Protobuf?

    internal init(protobuf proto: Wrapped.Protobuf?) {
        self = proto.map(Wrapped.fromProtobuf)
    }
}

extension Array: TryFromProtobuf where Element: TryFromProtobuf {
    internal typealias Protobuf = [Element.Protobuf]

    internal init(protobuf proto: [Element.Protobuf]) throws {
        self = try proto.map(Element.fromProtobuf)
    }
}

extension Array: FromProtobuf where Element: FromProtobuf {
    internal typealias Protobuf = [Element.Protobuf]

    internal init(protobuf proto: [Element.Protobuf]) {
        self = proto.map(Element.fromProtobuf)
    }
}
