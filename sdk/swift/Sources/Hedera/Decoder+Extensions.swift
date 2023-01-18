import Foundation

extension KeyedDecodingContainer {
    internal func decodeIfPresent<T: Decodable>(_ key: Key) throws -> T? {
        try decodeIfPresent(T.self, forKey: key)
    }

    internal func decode<T: Decodable>(_ key: Key) throws -> T {
        try decode(T.self, forKey: key)
    }
}
