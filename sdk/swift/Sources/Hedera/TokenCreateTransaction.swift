import Foundation

/// Create a new token.
public class TokenCreateTransaction: Transaction {
    /// Create a new `TokenCreateTransaction`.
    public init(
        name: String = "",
        symbol: String = "",
        decimals: UInt32 = 0,
        initialSupply: UInt64 = 0,
        treasuryAccountId: AccountAddress? = nil,
        adminKey: Key? = nil,
        kycKey: Key? = nil,
        freezeKey: Key? = nil,
        wipeKey: Key? = nil,
        supplyKey: Key? = nil,
        freezeDefault: Bool = false,
        expiresAt: Date? = nil,
        autoRenewAccountId: AccountAddress? = nil,
        autoRenewPeriod: TimeInterval? = nil,
        tokenMemo: String = "",
        tokenType: TokenType = .fungibleCommon,
        tokenSupplyType: TokenSupplyType = .infinite,
        maxSupply: UInt64 = 0,
        feeScheduleKey: Key? = nil,
        customFees: [CustomFee] = [],
        pauseKey: Key? = nil
    ) {
        self.name = name
        self.symbol = symbol
        self.decimals = decimals
        self.initialSupply = initialSupply
        self.treasuryAccountId = treasuryAccountId
        self.adminKey = adminKey
        self.kycKey = kycKey
        self.freezeKey = freezeKey
        self.wipeKey = wipeKey
        self.supplyKey = supplyKey
        self.freezeDefault = freezeDefault
        self.expiresAt = expiresAt
        self.autoRenewAccountId = autoRenewAccountId
        self.autoRenewPeriod = autoRenewPeriod
        self.tokenMemo = tokenMemo
        self.tokenType = tokenType
        self.tokenSupplyType = tokenSupplyType
        self.maxSupply = maxSupply
        self.feeScheduleKey = feeScheduleKey
        self.customFees = customFees
        self.pauseKey = pauseKey
    }

    /// The publicly visible name of the token.
    public var name: String

    /// The publicly visible token symbol.
    public var symbol: String

    /// The number of decimal places a fungible token is divisible by.
    public var decimals: UInt32

    /// The initial supply of fungible tokens to to mint to the treasury account.
    public var initialSupply: UInt64

    /// The account which will act as a treasury for the token.
    public var treasuryAccountId: AccountAddress?

    /// The key which can perform update/delete operations on the token.
    public var adminKey: Key?

    /// The key which can grant or revoke KYC of an account for the token's transactions.
    public var kycKey: Key?

    /// The key which can sign to freeze or unfreeze an account for token transactions.
    public var freezeKey: Key?

    /// The key which can wipe the token balance of an account.
    public var wipeKey: Key?

    /// The key which can change the supply of a token.
    public var supplyKey: Key?

    /// The default freeze status (frozen or unfrozen) of Hedera accounts relative to this token. If
    /// true, an account must be unfrozen before it can receive the token.
    public var freezeDefault: Bool

    /// The time at which the token should expire.
    public var expiresAt: Date?

    /// An account which will be automatically charged to renew the token's expiration, at
    /// `autoRenewPeriod` interval.
    public var autoRenewAccountId: AccountAddress?

    /// The interval at which the auto-renew account will be charged to extend the token's expiry.
    public var autoRenewPeriod: TimeInterval?

    /// The memo associated with the token.
    public var tokenMemo: String

    /// The token type. Defaults to FungibleCommon.
    public var tokenType: TokenType

    /// The token supply type. Defaults to Infinite.
    public var tokenSupplyType: TokenSupplyType

    /// Sets the maximum number of tokens that can be in circulation.
    public var maxSupply: UInt64

    /// The key which can change the token's custom fee schedule.
    public var feeScheduleKey: Key?

    /// The custom fees to be assessed during a transfer.
    public var customFees: [CustomFee]

    /// The key which can pause and unpause the token.
    public var pauseKey: Key?

    private enum CodingKeys: String, CodingKey {
        case name
        case symbol
        case decimals
        case initialSupply
        case treasuryAccountId
        case adminKey
        case kycKey
        case freezeKey
        case wipeKey
        case supplyKey
        case freezeDefault
        case expiresAt
        case autoRenewAccountId
        case autoRenewPeriod
        case tokenMemo
        case tokenType
        case tokenSupplyType
        case maxSupply
        case feeScheduleKey
        case customFees
        case pauseKey
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(name, forKey: .name)
        try container.encode(symbol, forKey: .symbol)
        try container.encode(decimals, forKey: .decimals)
        try container.encode(initialSupply, forKey: .initialSupply)
        try container.encodeIfPresent(treasuryAccountId, forKey: .treasuryAccountId)
        try container.encodeIfPresent(adminKey, forKey: .adminKey)
        try container.encodeIfPresent(kycKey, forKey: .kycKey)
        try container.encodeIfPresent(freezeKey, forKey: .freezeKey)
        try container.encodeIfPresent(wipeKey, forKey: .wipeKey)
        try container.encodeIfPresent(supplyKey, forKey: .supplyKey)
        try container.encode(freezeDefault, forKey: .freezeDefault)
        try container.encodeIfPresent(expiresAt, forKey: .expiresAt)
        try container.encodeIfPresent(autoRenewAccountId, forKey: .autoRenewAccountId)
        try container.encodeIfPresent(autoRenewPeriod, forKey: .autoRenewPeriod)
        try container.encode(tokenMemo, forKey: .tokenMemo)
        try container.encode(tokenType, forKey: .tokenType)
        try container.encode(tokenSupplyType, forKey: .tokenSupplyType)
        try container.encode(maxSupply, forKey: .maxSupply)
        try container.encodeIfPresent(feeScheduleKey, forKey: .feeScheduleKey)
        try container.encode(customFees, forKey: .customFees)
        try container.encodeIfPresent(pauseKey, forKey: .pauseKey)

        try super.encode(to: encoder)
    }
}
