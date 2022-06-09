import com.hedera.hashgraph.sdk.AccountBalanceQuery;
import com.hedera.hashgraph.sdk.AccountId;
import com.hedera.hashgraph.sdk.Client;

class GetAccountBalanceExample {
    public static void main(String[] args) {
        var client = Client.forTestnet();

        var query = new AccountBalanceQuery();
        query.setAccountId(AccountId.parse("0.0.1001"));

        var response = query.execute(client);

        System.out.printf("balance = %s\n", response.balance);
    }
}
