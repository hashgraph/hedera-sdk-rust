/// Information about a single account that is proxy staking.
public struct ProxyStaker: Codable {
    /// The Account ID that is proxy staking.
    public let accountId: AccountId

    /// The number of hbars that are currently proxy staked.
    public let amount: UInt64
}
