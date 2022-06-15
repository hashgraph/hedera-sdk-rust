import com.hedera.hashgraph.sdk.AccountCreateTransaction;
import com.hedera.hashgraph.sdk.AccountId;
import com.hedera.hashgraph.sdk.Client;
import com.hedera.hashgraph.sdk.PrivateKey;

class CreateAccountExample {
    public static void main(String[] args) {
        var client = Client.forTestnet()
            .setPayerAccountId(AccountId.parse("0.0.6189"))
            .addDefaultSigner(PrivateKey.parse("7f7ac6c8025a15ff1e07ef57c7295601379a4e9a526560790ae85252393868f0"));

        var newKey = PrivateKey.generateEd25519();

        System.out.printf("private key = %s\n", newKey);
        System.out.printf("public key = %s\n", newKey.getPublicKey());

        var response = new AccountCreateTransaction()
            .setKey(newKey.getPublicKey())
            .execute(client);

        var receipt = response.getReceipt(client);

        System.out.printf("account address = %s\n", receipt.accountId);
    }
}
