import Foundation
import SwiftProtobuf

internal protocol ToProtobuf {
    associatedtype Protobuf

    func toProtobuf() -> Protobuf

    func toProtobufInto(_ place: inout Protobuf)
}

extension ToProtobuf {
    internal func toProtobufInto(_ place: inout Protobuf) {
        place = toProtobuf()
    }
}

extension ToProtobuf where Protobuf: Message {
    internal func toProtobufBytes() -> Data {
        // this is a force try because in theory we should never fail and the user definitely can't do anything if we do fail.
        // swiftlint:disable:next force_try
        try! toProtobuf().serializedData()
    }
}

extension Array: ToProtobuf where Element: ToProtobuf {
    internal typealias Protobuf = [Element.Protobuf]

    internal func toProtobuf() -> [Element.Protobuf] {
        map { $0.toProtobuf() }
    }
}
