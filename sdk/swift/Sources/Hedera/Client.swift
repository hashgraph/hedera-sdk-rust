import CHedera

/// Managed client for use on the Hedera network.
public class Client {
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

    /// Adds a signer that will, by default, sign for all transactions and queries built
    /// with this client.
    public func addDefaultSigner(_ privateKey: PrivateKey) {
        hedera_client_add_default_signer(ptr, hedera_signer_private_key(privateKey.ptr))
    }
}
