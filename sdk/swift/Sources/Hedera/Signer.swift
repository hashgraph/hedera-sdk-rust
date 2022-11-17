import CHedera
import Foundation

func sign(
    _ context: UnsafeMutableRawPointer?, _ message: UnsafePointer<UInt8>?, _ messageSize: size_t,
    _ signature: UnsafeMutablePointer<UnsafePointer<UInt8>?>?
) -> size_t {
    let signer = Unmanaged<Signer>.fromOpaque(context!).takeUnretainedValue()

    // have to copy; we don't have a mutable pointer
    let messageData = Data(bytes: message!, count: messageSize)

    let signatureData = signer.signFunc(messageData)

    // raw pointer timeeee (yes, we do need to double copy.)
    let buffer = signatureData.withUnsafeBytes { dataBuffer -> UnsafeMutableBufferPointer<UInt8> in
        let buffer = UnsafeMutableRawBufferPointer.allocate(byteCount: dataBuffer.count, alignment: 1)
        buffer.copyBytes(from: dataBuffer)
        return buffer.bindMemory(to: UInt8.self)
    }

    signature!.pointee = hackMutablePointerToImmutable(buffer.baseAddress)
    return buffer.count
}

func freeSignature(
    _ context: UnsafeMutableRawPointer?, _ signature: UnsafeMutablePointer<UInt8>?, _ signatureSize: size_t
) {
    UnsafeMutableBufferPointer(start: signature!, count: signatureSize).deallocate()
}

func freeContext(_ context: UnsafeMutableRawPointer?) {
    let _ = Unmanaged<Signer>.fromOpaque(context!).takeRetainedValue()
}

// note(sr): swift understands this, but it doesn't understand if I were to just do the `signature!.pointee = buffer.baseAddress`, seriously, feel free to try it.
func hackMutablePointerToImmutable<Pointee>(_ ptr: UnsafePointer<Pointee>?) -> UnsafePointer<Pointee>? {
    ptr
}

public class Signer {
    internal init(_ publicKey: PublicKey, _ signFunc: @escaping (Data) -> Data) {
        self.publicKey = publicKey
        self.signFunc = signFunc
    }

    let publicKey: PublicKey
    let signFunc: (Data) -> Data

    internal func unsafeIntoHederaSigner() -> HederaSigner {
        let unmanaged = Unmanaged.passRetained(self)
        let ptr = unmanaged.toOpaque()
        return HederaSigner(
            public_key: self.publicKey.ptr, context: ptr, sign_func: sign, free_signature_func: freeSignature,
            free_context_func: freeContext)
    }
}

private func freeHederaSigners(_ ptr: UnsafePointer<HederaSigner>?, _ size: size_t) {
    UnsafeBufferPointer(start: ptr!, count: size).deallocate()
}

internal func makeHederaSignersFromArray(signers: [Signer]) -> HederaSigners {
    let buffer = UnsafeMutableBufferPointer<HederaSigner>.allocate(capacity: signers.count)

    var (iterator_rest, idx) = buffer.initialize(from: signers.makeIterator().map { $0.unsafeIntoHederaSigner() })

    assert(iterator_rest.next() == nil)
    assert(idx == signers.count)

    return HederaSigners(signers: buffer.baseAddress, signers_size: buffer.count, free: freeHederaSigners)
}
