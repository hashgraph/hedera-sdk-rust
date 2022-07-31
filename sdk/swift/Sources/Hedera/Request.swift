import CHedera
import Foundation

/// A transaction or query that can be executed on the Hedera network.
public protocol Request: Encodable {
    associatedtype Response: Decodable

    func execute(_ client: Client) async throws -> Response

    func decodeResponse(_ responseBytes: Data) throws -> Response
}

extension Request {
    /// Execute this request against the provided client of the Hedera network.
    public func execute(_ client: Client) async throws -> Response {
        // encode self as a JSON request to pass to Rust
        let requestBytes = try JSONEncoder().encode(self)
        let request = String(data: requestBytes, encoding: .utf8)!

        // start an unmanaged continuation to bridge a C callback with Swift async
        let responseBytes: Data = try await withUnmanagedThrowingContinuation { continuation in
            // invoke `hedera_execute`, callback will be invoked on request completion
            let err = hedera_execute(client.ptr, request, continuation) { continuation, err, responsePtr in
                if err != HEDERA_ERROR_OK {
                    // an error has occurred, consume from the TLS storage for the error
                    // and throw it up back to the async task
                    resumeUnmanagedContinuation(continuation, throwing: HError(err)!)
                } else {
                    // NOTE: we are guaranteed to receive valid UTF-8 on a successful response
                    let responseText = String(validatingUTF8: responsePtr!)!
                    let responseBytes = responseText.data(using: .utf8)!

                    // resumes the continuation which bridges us back into Swift async
                    resumeUnmanagedContinuation(continuation, returning: responseBytes)
                }
            }

            if err != HEDERA_ERROR_OK {
                resumeUnmanagedContinuation(continuation, throwing: HError(err)!)
            }
        }

        return try decodeResponse(responseBytes)
    }

    public func decodeResponse(_ responseBytes: Data) throws -> Response {
        // decode the response as the generic output type of this query types
        try JSONDecoder().decode(Response.self, from: responseBytes)
    }
}
