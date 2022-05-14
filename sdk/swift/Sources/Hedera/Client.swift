import CHedera

/// Managed client for use on the Hedera network.
public class Client {
    // TODO: is there a better way to share this everywhere?
    let ptr: OpaquePointer

    private init(_ ptr: OpaquePointer) {
        self.ptr = ptr
    }

    deinit {
        hedera_client_free(ptr)
    }

    /// Construct a Hedera client pre-configured for testnet access.
    public static func forTestnet() -> Client {
        Client(hedera_client_for_testnet())
    }

    /// Sets the account that will, by default, be paying for transactions and queries built with
    /// this client.
    public func setPayerAccountId(_ accountId: AccountId) {
        hedera_client_set_payer_account_id(
            ptr,
            HederaAccountId(
                shard: accountId.shard,
                realm: accountId.realm,
                num: accountId.num
            ))
    }
}
