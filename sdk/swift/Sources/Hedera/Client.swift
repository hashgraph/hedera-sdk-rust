import CHedera

/// Managed client for use on the Hedera network.
public final class Client {
    internal let ptr: OpaquePointer

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
    public func setOperator(_ accountId: AccountId, _ privateKey: PrivateKey) {
        hedera_client_set_operator(
            ptr, accountId.shard, accountId.realm, accountId.num, privateKey.ptr)
    }
}
