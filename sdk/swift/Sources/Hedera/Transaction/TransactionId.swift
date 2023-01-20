import CHedera
import Foundation

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

    internal func unsafeWithCHedera<Result>(_ body: (HederaTransactionId) throws -> Result) rethrows -> Result {
        try accountId.unsafeWithCHedera { hederaAccountId in
            try body(
                HederaTransactionId(
                    account_id: hederaAccountId, valid_start: validStart.toCHederaTimestamp(), nonce: nonce ?? 0,
                    scheduled: scheduled))
        }
    }

    private init(parsing description: String) throws {
        // this is extra painful in swift...
        let expected = "expecting <accountId>@<validStart>[?scheduled][/<nonce>]"
        // parse route:
        // split_once('@') -> ("<accountId>", "<validStart>[?scheduled][/<nonce>]")
        // rsplit_once('/') -> Either ("<validStart>[?scheduled]", "<nonce>") or ("<validStart>[?scheduled]")
        // .strip_suffix("?scheduled") -> ("<validStart>") and the suffix was either removed or not.
        // (except it's better ux to do a `split_once('?')`... Except it doesn't matter that much)

        guard let tmp = description.splitOnce(on: "@") else {
            throw HError.basicParse(expected)
        }

        let accountId = try AccountId(parsing: tmp.0)

        let (description, nonceStr) = tmp.1.rsplitOnce(on: "/") ?? (tmp.0, nil)

        let nonce = try nonceStr.map(Int32.init(parsing:))

        let (validStartStr, scheduled) =
            description.stripSuffix("?scheduled").map { ($0, true) } ?? (description, false)

        guard let (validStartSeconds, validStartNanos) = validStartStr.splitOnce(on: ".") else {
            throw HError.basicParse(expected)
        }

        let validStart = try Timestamp(
            seconds: UInt64(parsing: validStartSeconds), subSecondNanos: UInt32(parsing: validStartNanos)
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
        try bytes.withUnsafeTypedBytes { pointer in
            var id = HederaTransactionId()

            try HError.throwing(error: hedera_transaction_id_from_bytes(pointer.baseAddress, pointer.count, &id))

            return Self(unsafeFromCHedera: id)
        }
    }

    public func toBytes() -> Data {
        unsafeWithCHedera { hedera in
            var buf: UnsafeMutablePointer<UInt8>?
            let size = hedera_transaction_id_to_bytes(hedera, &buf)

            return Data(bytesNoCopy: buf!, count: size, deallocator: .unsafeCHederaBytesFree)
        }
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

    //     public var foo: Bar {
    //     // `ensureNotFrozen` is a bikesheddable, need to find a balance between "this will die" and "name that's short enough"
    //     // for instance:
    //     // precondition(!frozen, "`foo` cannot be set while `\(String(describing: type(of: self)))` is frozen")
    //     // gives wonderful information, but is also really long
    //     // whereas:
    //     // ensureNotFrozen(fieldName: "foo")
    //     // is a decent chunk shorter and should be able to give all the same info.
    //     willSet { ensureNotFrozen() }
    // }
}
