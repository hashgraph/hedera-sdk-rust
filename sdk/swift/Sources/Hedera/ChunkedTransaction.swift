import CHedera
import Foundation
import GRPC

public class ChunkedTransaction: Transaction {
    internal static let defaultMaxChunks = 20
    internal static let defaultChunkSize = 1024

    public override init() {
        super.init()
    }

    internal init(data: Data) {
        self.data = data
        super.init()
    }

    public required init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        data = try container.decodeIfPresent(.data).map(Data.base64Encoded) ?? Data()
        maxChunks = try container.decodeIfPresent(.maxChunks) ?? Self.defaultMaxChunks
        chunkSize = try container.decodeIfPresent(.chunkSize) ?? Self.defaultChunkSize

        try super.init(from: decoder)
    }

    /// Message/contents for this transaction.
    ///
    /// Please expose this under the appropriate name (with the frozen check) when inheriting from this class.
    internal final var data: Data = Data() {
        willSet {
            // note: This exists in case one is forgotten, prefer the `willSet` elsewhere.
            ensureNotFrozen(fieldName: "data")
        }
    }

    /// The maximum number of chunks this transaction will be split into.
    public final var maxChunks = defaultMaxChunks {
        willSet {
            ensureNotFrozen(fieldName: "maxChunks")
        }
    }

    /// Sets the maximum number of chunks this transaction will be split into.
    @discardableResult
    public final func maxChunks(_ maxChunks: Int) -> Self {
        self.maxChunks = maxChunks

        return self
    }

    /// The maximum size of any chunk of this transaction.
    public final var chunkSize = defaultChunkSize {
        willSet {
            ensureNotFrozen(fieldName: "chunkSize")
        }
    }

    /// Sets the maximum size of any chunk of this transaction.
    @discardableResult
    public final func chunkSize(_ chunkSize: Int) -> Self {
        precondition(chunkSize != 0)

        self.chunkSize = chunkSize

        return self
    }

    internal final var usedChunks: Int {
        if data.isEmpty {
            return 1
        }

        // div ceil algorithm
        return (data.count + chunkSize) / chunkSize
    }

    fileprivate final var maxMessageSize: Int { maxChunks * chunkSize }

    internal var waitForReceipt: Bool { false }

    internal final func messageChunk(_ chunkInfo: ChunkInfo) -> Data {
        assert(chunkInfo.current < usedChunks)

        let start = self.chunkSize * chunkInfo.current;
        let end = min(self.chunkSize * (chunkInfo.current + 1), self.data.count);

        return self.data[start..<end]

    }

    private enum CodingKeys: CodingKey {
        case data
        case maxChunks
        case chunkSize
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(data, forKey: .data)

        if maxChunks != Self.defaultMaxChunks {
            try container.encode(maxChunks, forKey: .maxChunks)
        }

        if chunkSize != Self.defaultChunkSize {
            try container.encode(chunkSize, forKey: .chunkSize)
        }

        try super.encode(to: encoder)
    }

    internal final override func addSignatureSigner(_ signer: Signer) {
        precondition(
            self.usedChunks <= 1,
            "cannot manually add a signature to a chunked transaction with multiple chunks (message length `\(data.count)` > chunk size `\(chunkSize)`)"
        )

        super.addSignatureSigner(signer)
    }

    public final override func execute(_ client: Client, _ timeout: TimeInterval? = nil) async throws
        -> TransactionResponse
    {
        // note: this could be called directly from something that sees a `Transaction`
        try await executeAll(client, timeout)[0]
    }

    public final func executeAll(_ client: Client, _ timeoutPerChunk: TimeInterval? = nil) async throws
        -> [TransactionResponse]
    {
        try freezeWith(client)

        precondition(self.data.count < self.maxMessageSize, "todo: throw an actual error here")

        var responses: [Response] = []

        let initialTransactionId: TransactionId

        let usedChunks = self.usedChunks

        do {
            let resp = try await executeAny(
                client,
                FirstChunkView(transaction: self, totalChunks: usedChunks),
                timeoutPerChunk
            )

            if waitForReceipt {
                _ = try await resp.getReceipt(client, timeoutPerChunk)
            }

            initialTransactionId = resp.transactionId

            responses.append(resp)
        }

        for chunk in 1..<usedChunks {
            let resp = try await executeAny(
                client,
                ChunkView(
                    transaction: self, initialTransactionId: initialTransactionId, currentChunk: chunk,
                    totalChunks: usedChunks),
                timeoutPerChunk
            )

            if waitForReceipt {
                _ = try await resp.getReceipt(client, timeoutPerChunk)
            }

            responses.append(resp)

        }

        return responses
    }
}

extension ChunkedTransaction {
    fileprivate struct FirstChunkView<Tx: ChunkedTransaction> {
        let transaction: Tx
        let totalChunks: Int
    }

    fileprivate struct ChunkView<Tx: ChunkedTransaction> {
        let transaction: Tx
        let initialTransactionId: TransactionId
        let currentChunk: Int
        let totalChunks: Int
    }
}

extension ChunkedTransaction.FirstChunkView: Execute {
    internal typealias GrpcRequest = Tx.GrpcRequest
    internal typealias GrpcResponse = Tx.GrpcResponse
    internal typealias Context = Tx.Context
    internal typealias Response = Tx.Response

    internal var nodeAccountIds: [AccountId]? { transaction.nodeAccountIds }
    internal var explicitTransactionId: TransactionId? { transaction.transactionId }
    internal var requiresTransactionId: Bool { true }

    internal func makeRequest(_ transactionId: TransactionId?, _ nodeAccountId: AccountId) throws -> (
        GrpcRequest, Context
    ) {
        assert(transaction.isFrozen)

        guard let transactionId = transactionId else {
            throw HError.noPayerAccountOrTransactionId
        }

        return transaction.makeRequestInner(
            chunkInfo: .initial(total: totalChunks, transactionId: transactionId, nodeAccountId: nodeAccountId)
        )
    }

    internal func execute(_ channel: GRPCChannel, _ request: GrpcRequest) async throws -> GrpcResponse {
        try await transaction.transactionExecute(channel, request)
    }

    internal func makeResponse(
        _ response: GrpcResponse, _ context: Context, _ nodeAccountId: AccountId, _ transactionId: TransactionId?
    ) -> Response {
        transaction.makeResponse(response, context, nodeAccountId, transactionId)
    }

    internal func makeErrorPrecheck(_ status: Status, _ transactionId: TransactionId?) -> HError {
        transaction.makeErrorPrecheck(status, transactionId)
    }

    internal static func responsePrecheckStatus(_ response: GrpcResponse) throws -> Int32 {
        Tx.responsePrecheckStatus(response)
    }
}

extension ChunkedTransaction.ChunkView: Execute {
    internal typealias GrpcRequest = Tx.GrpcRequest
    internal typealias GrpcResponse = Tx.GrpcResponse
    internal typealias Context = Tx.Context
    internal typealias Response = Tx.Response

    internal var nodeAccountIds: [AccountId]? { transaction.nodeAccountIds }
    internal var explicitTransactionId: TransactionId? { nil }
    internal var requiresTransactionId: Bool { true }

    internal func makeRequest(_ transactionId: TransactionId?, _ nodeAccountId: AccountId) throws -> (
        GrpcRequest, Context
    ) {
        assert(transaction.isFrozen)

        guard let transactionId = transactionId else {
            throw HError.noPayerAccountOrTransactionId
        }

        return transaction.makeRequestInner(
            chunkInfo: .init(
                current: currentChunk,
                total: totalChunks,
                initialTransactionId: initialTransactionId,
                currentTransactionId: transactionId,
                nodeAccountId: nodeAccountId
            )
        )
    }

    internal func execute(_ channel: GRPCChannel, _ request: GrpcRequest) async throws -> GrpcResponse {
        try await transaction.transactionExecute(channel, request)
    }

    internal func makeResponse(
        _ response: GrpcResponse, _ context: Context, _ nodeAccountId: AccountId, _ transactionId: TransactionId?
    ) -> Response {
        transaction.makeResponse(response, context, nodeAccountId, transactionId)
    }

    internal func makeErrorPrecheck(_ status: Status, _ transactionId: TransactionId?) -> HError {
        transaction.makeErrorPrecheck(status, transactionId)
    }

    internal static func responsePrecheckStatus(_ response: GrpcResponse) throws -> Int32 {
        Tx.responsePrecheckStatus(response)
    }
}

extension ChunkedTransaction.FirstChunkView: ValidateChecksums {
    func validateChecksums(on ledgerId: LedgerId) throws {
        try self.transaction.validateChecksums(on: ledgerId)
    }
}

extension ChunkedTransaction.ChunkView: ValidateChecksums {
    func validateChecksums(on ledgerId: LedgerId) throws {
        try self.transaction.validateChecksums(on: ledgerId)
        try self.initialTransactionId.validateChecksums(on: ledgerId)
    }
}
