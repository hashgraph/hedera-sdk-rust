import Foundation
import SwiftProtobuf

internal protocol ToProtobuf {
    associatedtype Protobuf

    func toProtobuf() -> Protobuf
}

extension ToProtobuf where Protobuf: Message {
    internal func toProtobufBytes() -> Data {
        try! toProtobuf().serializedData()
    }
}

extension Array: ToProtobuf where Element: ToProtobuf {
    internal typealias Protobuf = [Element.Protobuf]

    internal func toProtobuf() -> [Element.Protobuf] {
        map { $0.toProtobuf() }
    }
}
