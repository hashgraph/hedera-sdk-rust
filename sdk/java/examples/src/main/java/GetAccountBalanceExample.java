import com.hedera.hashgraph.sdk.AccountBalanceQuery;
import com.hedera.hashgraph.sdk.AccountId;
import com.hedera.hashgraph.sdk.Client;

class GetAccountBalanceExample {
    public static void main(String[] args) {
        var client = Client.forTestnet();

        var response = new AccountBalanceQuery()
            .setAccountId(AccountId.parse("0.0.1001"))
            .execute(client);

        System.out.printf("balance = %s\n", response.balance);
    }
}
