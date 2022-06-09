@file:JvmName("GetAccountBalanceKtExample")

import com.hedera.hashgraph.sdk.AccountBalanceQuery
import com.hedera.hashgraph.sdk.AccountId
import com.hedera.hashgraph.sdk.Client

fun main() {
    val client = Client.forTestnet()

    val response = AccountBalanceQuery(
        accountId = AccountId.parse("0.0.1001")
    ).execute(client)

    println("balance = ${response.balance}")
}
