import CHedera
import Foundation
import HederaProtobufs

public struct TransactionId: Codable, Equatable, ExpressibleByStringLiteral, LosslessStringConvertible,
    ValidateChecksums
{
    /// The Account ID that paid for this transaction.
    public let accountId: AccountId

    /// The time from when this transaction is valid.
    ///
    /// When a transaction is submitted there is additionally a validDuration (defaults to 120s)
    /// and together they define a time window that a transaction may be processed in.
    public let validStart: Timestamp
    public let scheduled: Bool
    public let nonce: Int32?

    internal init(accountId: AccountId, validStart: Timestamp, scheduled: Bool, nonce: Int32? = nil) {
        self.accountId = accountId
        self.validStart = validStart
        self.scheduled = scheduled
        self.nonce = nonce
    }

    internal init(unsafeFromCHedera hedera: HederaTransactionId) {
        accountId = AccountId(unsafeFromCHedera: hedera.account_id)
        validStart = Timestamp(fromCHedera: hedera.valid_start)
        nonce = hedera.nonce != 0 ? hedera.nonce : nil
        scheduled = hedera.scheduled
    }

    private init<S: StringProtocol>(parsing description: S) throws {
        let expected = "expecting <accountId>@<validStart>[?scheduled][/<nonce>]"

        // parse route:
        // splitOnce('@') -> ("<accountId>", "<validStart>[?scheduled][/<nonce>]")
        // rsplitOnce('/') -> Either ("<validStart>[?scheduled]", "<nonce>") or ("<validStart>[?scheduled]")
        // .stripSuffix("?scheduled") -> ("<validStart>") and the suffix was either removed or not.
        // (except it's better ux to do a `splitOnce('?')`... Except it doesn't matter that much)
        guard let (accountIdStr, rest) = description.splitOnce(on: "@") else {
            throw HError(kind: .basicParse, description: expected)
        }

        let accountId = try AccountId(parsing: accountIdStr)

        let (rest2, nonceStr) = rest.rsplitOnce(on: "/") ?? (rest, nil)

        let nonce = try nonceStr.map(Int32.init(parsing:))

        let validStartStr: S.SubSequence
        let scheduled: Bool

        switch rest2.stripSuffix("?scheduled") {
        case .some(let value):
            validStartStr = value
            scheduled = true
        case nil:
            validStartStr = rest2
            scheduled = false
        }

        guard let (validStartSeconds, validStartNanos) = validStartStr.splitOnce(on: ".") else {
            throw HError(kind: .basicParse, description: expected)
        }

        let validStart = Timestamp(
            seconds: try UInt64(parsing: validStartSeconds),
            subSecondNanos: try UInt32(parsing: validStartNanos)
        )

        self.init(accountId: accountId, validStart: validStart, scheduled: scheduled, nonce: nonce)
    }

    public static func fromString(_ description: String) throws -> Self {
        try Self(parsing: description)
    }

    public init(stringLiteral value: StringLiteralType) {
        // swiftlint:disable:next force_try
        try! self.init(parsing: value)
    }

    // txid parsing is shockingly hard. So the FFI really does carry its weight.
    public init?(_ description: String) {
        try? self.init(parsing: description)
    }

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(protobufBytes: bytes)
    }

    public func toBytes() -> Data {
        self.toProtobufBytes()
    }

    public var description: String {
        let scheduled = scheduled ? "?scheduled" : ""
        let nonce = nonce.map { "/\($0)" } ?? ""
        return
            "\(accountId)@\(validStart.seconds).\(validStart.subSecondNanos)\(scheduled)\(nonce)"
    }

    public func toString() -> String {
        String(describing: self)
    }

    public init(from decoder: Decoder) throws {
        self.init(try decoder.singleValueContainer().decode(String.self))!
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        try container.encode(String(describing: self))
    }

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try accountId.validateChecksums(on: ledgerId)
    }
}

extension TransactionId: TryProtobufCodable {
    internal typealias Protobuf = Proto_TransactionID

    internal init(protobuf proto: Protobuf) throws {
        self.init(
            accountId: try .fromProtobuf(proto.accountID),
            validStart: .fromProtobuf(proto.transactionValidStart),
            scheduled: proto.scheduled,
            nonce: proto.nonce != 0 ? proto.nonce : nil
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.accountID = accountId.toProtobuf()
            proto.transactionValidStart = validStart.toProtobuf()
            proto.scheduled = scheduled
            proto.nonce = nonce ?? 0
        }
    }
}
