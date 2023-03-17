/*
 * ‌
 * Hedera Swift SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

/// Suspends the current task, then calls the given closure with an unmanaged continuation for the current task.
internal func withUnmanagedContinuation<T>(_ fun: (UnsafeRawPointer) -> Void) async -> T {
    await withUnsafeContinuation { (continuation: UnsafeContinuation<T, Never>) in
        let continuationHandle = ContinuationHandle(continuation)
        let continuationPtr = Unmanaged.passRetained(continuationHandle).toOpaque()

        fun(continuationPtr)
    }
}

/// Suspends the current task, then calls the given closure with an unmanaged continuation for the current task.
internal func withUnmanagedThrowingContinuation<T>(_ fun: (UnsafeRawPointer) -> Void) async throws -> T {
    try await withUnsafeThrowingContinuation { (continuation: UnsafeContinuation<T, Error>) in
        let continuationHandle = ContinuationHandle(continuation)
        let continuationPtr = Unmanaged.passRetained(continuationHandle).toOpaque()

        fun(continuationPtr)
    }
}

/// Resumes the current task.
/// Must be called with a pointer that was returned in the callback from ``withUnmanagedContinuation``.
internal func resumeUnmanagedContinuation(_ ptr: UnsafeRawPointer!) {
    let continuationHandle = Unmanaged<ContinuationHandle<Void, Never>>.fromOpaque(ptr!)
        .takeUnretainedValue()

    let continuation = continuationHandle.continuation

    continuation.resume()
}

/// Resumes the current task with the given success.
/// Must be called with a pointer that was returned in the callback from ``withUnmanagedThrowingContinuation``.
internal func resumeUnmanagedContinuation<T>(
    _ ptr: UnsafeRawPointer!, returning value: T
) {
    let continuationHandle = Unmanaged<ContinuationHandle<T, Error>>.fromOpaque(ptr!)
        .takeUnretainedValue()

    let continuation = continuationHandle.continuation

    continuation.resume(returning: value)
}

/// Resumes the current task with the given failure.
/// Must be called with a pointer that was returned in the callback from ``withUnmanagedThrowingContinuation``.
internal func resumeUnmanagedContinuation(
    _ ptr: UnsafeRawPointer!, throwing error: Error
) {
    let continuationHandle = Unmanaged<ContinuationHandle<Never, Error>>.fromOpaque(ptr!)
        .takeUnretainedValue()

    let continuation = continuationHandle.continuation

    continuation.resume(throwing: error)
}

private final class ContinuationHandle<T, E: Error> {
    let continuation: UnsafeContinuation<T, E>

    init(_ continuation: UnsafeContinuation<T, E>) {
        self.continuation = continuation
    }
}
