import CHedera
import Foundation

internal typealias FromJsonBytesFunc = @convention(c)
(_ bytes: UnsafePointer<UInt8>?, _ bytes_size: Int, _ s: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) ->
    HederaError

internal typealias ToJsonBytesFunc = @convention(c)
(
    _ s: UnsafePointer<CChar>?, _ buf: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    _ buf_size: UnsafeMutablePointer<Int>?
) -> HederaError

internal protocol FromJsonBytes: Decodable {
    static var cFromBytes: FromJsonBytesFunc { get }
    static func fromJsonBytes(_ bytes: Data) throws -> Self
}

internal protocol ToJsonBytes: Encodable {
    static var cToBytes: ToJsonBytesFunc { get }
    func toJsonBytes() throws -> Data
}

internal typealias ToFromJsonBytes = ToJsonBytes & FromJsonBytes

extension FromJsonBytes {
    internal static func fromJsonBytes(_ bytes: Data) throws -> Self {
        let json: String = try bytes.withUnsafeTypedBytes { pointer in
            var ptr: UnsafeMutablePointer<CChar>?
            try HError.throwing(error: cFromBytes(pointer.baseAddress, pointer.count, &ptr))

            return String(hString: ptr!)
        }

        return try JSONDecoder().decode(Self.self, from: json.data(using: .utf8)!)
    }
}

extension ToJsonBytes {
    internal func toJsonBytes() throws -> Data {
        let jsonBytes = try JSONEncoder().encode(self)
        let json = String(data: jsonBytes, encoding: .utf8)!
        var buf: UnsafeMutablePointer<UInt8>?
        var bufSize: Int = 0

        try HError.throwing(error: Self.cToBytes(json, &buf, &bufSize))

        return Data(bytesNoCopy: buf!, count: bufSize, deallocator: .unsafeCHederaBytesFree)
    }
}
